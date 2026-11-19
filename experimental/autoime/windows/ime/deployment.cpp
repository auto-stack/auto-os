// AutoIME - resource deployment and recovery implementation (PLAN-070 T-04).
#include "deployment.h"

#include <windows.h>

#include <cstdio>
#include <cstring>

#include "diag.h"

namespace deployment {
namespace {

const char* const kManaged[] = {
    "autoime_pinyin.schema.yaml",
    "autoime_terms.schema.yaml",
    "autoime_en.schema.yaml",
    "autoime_terms.dict.yaml",
    "autoime_en.dict.yaml",
};
const char* const kPhrase = "custom_phrase.txt";
const char* const kMarker = ".autoime-resources";
const char* const kPending = ".autoime-resources.pending";
const char* const kBackupDir = ".autoime-backup";
const char* const kStageDir = ".autoime-staging";
const char* const kBuildDir = "build";

std::string Join(const std::string& a, const std::string& b) {
  if (a.empty()) return b;
  if (b.empty()) return a;
  if (b[0] == '\\' || b[0] == '/') return b;
  if (a.back() == '\\' || a.back() == '/') return a + b;
  return a + "\\" + b;
}

bool FileExists(const std::string& path) {
  const DWORD attrs = GetFileAttributesA(path.c_str());
  return attrs != INVALID_FILE_ATTRIBUTES &&
         !(attrs & FILE_ATTRIBUTE_DIRECTORY);
}

bool DirExists(const std::string& path) {
  const DWORD attrs = GetFileAttributesA(path.c_str());
  return attrs != INVALID_FILE_ATTRIBUTES &&
         (attrs & FILE_ATTRIBUTE_DIRECTORY);
}

uint64_t Fnv1a(const char* data, size_t size) {
  uint64_t hash = 1469598103934665603ULL;
  for (size_t i = 0; i < size; ++i) {
    hash ^= static_cast<unsigned char>(data[i]);
    hash *= 1099511628211ULL;
  }
  return hash;
}

bool ReadAll(const std::string& path, std::string* out) {
  FILE* f = std::fopen(path.c_str(), "rb");
  if (!f) return false;
  out->clear();
  char buffer[4096];
  size_t n = 0;
  while ((n = std::fread(buffer, 1, sizeof(buffer), f)) > 0)
    out->append(buffer, n);
  std::fclose(f);
  return true;
}

bool WriteAll(const std::string& path, const std::string& data) {
  const std::string tmp = path + ".tmp";
  FILE* f = std::fopen(tmp.c_str(), "wb");
  if (!f) return false;
  const bool wrote = std::fwrite(data.data(), 1, data.size(), f) == data.size();
  std::fclose(f);
  if (!wrote) {
    DeleteFileA(tmp.c_str());
    return false;
  }
  return MoveFileExA(tmp.c_str(), path.c_str(),
                     MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH) != 0;
}

std::string VersionString(uint64_t digest) {
  char buffer[32];
  std::snprintf(buffer, sizeof(buffer), "r%016llx",
                static_cast<unsigned long long>(digest));
  return buffer;
}

std::string MarkerPath(const std::string& dir) { return Join(dir, kMarker); }

}  // namespace

bool RemoveTree(const std::string& path) {
  if (!DirExists(path)) {
    return DeleteFileA(path.c_str()) != 0 || !FileExists(path);
  }
  WIN32_FIND_DATAA fd{};
  HANDLE find = FindFirstFileA(Join(path, "*").c_str(), &fd);
  if (find != INVALID_HANDLE_VALUE) {
    do {
      const std::string name = fd.cFileName;
      if (name == "." || name == "..") continue;
      const std::string full = Join(path, name);
      if (fd.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) {
        RemoveTree(full);
      } else {
        SetFileAttributesA(full.c_str(), FILE_ATTRIBUTE_NORMAL);
        DeleteFileA(full.c_str());
      }
    } while (FindNextFileA(find, &fd));
    FindClose(find);
  }
  return RemoveDirectoryA(path.c_str()) != 0;
}

DeployLock::DeployLock() {
  const char* names[] = {"Local\\AutoIME.Rime.Deploy",
                         "Global\\AutoIME.Rime.Deploy"};
  for (const char* name : names) {
    handle_ = CreateMutexA(nullptr, FALSE, name);
    if (!handle_) continue;
    const DWORD wait = WaitForSingleObject(handle_, 30000);
    if (wait == WAIT_OBJECT_0 || wait == WAIT_ABANDONED) {
      acquired_ = true;
      return;
    }
    CloseHandle(handle_);
    handle_ = nullptr;
  }
}

DeployLock::~DeployLock() {
  if (handle_) {
    if (acquired_) ReleaseMutex(handle_);
    CloseHandle(handle_);
    handle_ = nullptr;
  }
}

std::string ResolveResourcesDir(const std::string& module_dir) {
  char env[MAX_PATH]{};
  const DWORD n = GetEnvironmentVariableA("AUTOIME_RESOURCES_DIR", env,
                                          MAX_PATH);
  if (n > 0 && n < MAX_PATH && DirExists(env)) return env;
  // PLAN-071 T-01: the shipped resources live in <autoime>\shared\resources;
  // the third suffix reaches it from the dev build tree (windows/ime/build).
  const char* const suffixes[] = {"\\resources", "\\..\\resources",
                                  "\\..\\..\\resources",
                                  "\\..\\..\\..\\shared\\resources"};
  for (const char* suffix : suffixes) {
    const std::string candidate = module_dir + suffix;
    if (DirExists(candidate)) return candidate;
  }
  return std::string();
}

std::string DefaultUserDir() {
  char env[MAX_PATH]{};
  DWORD n = GetEnvironmentVariableA("AUTOIME_RIME_USER_DATA_DIR", env,
                                    MAX_PATH);
  if (n > 0 && n < MAX_PATH) return env;
  n = GetEnvironmentVariableA("LOCALAPPDATA", env, MAX_PATH);
  const std::string base = (n > 0 && n < MAX_PATH) ? env : "";
  return Join(base, "AutoIME\\rime");
}

Manifest BuildManifest(const std::string& resources_dir, std::string* error) {
  Manifest manifest;
  std::string combined;
  for (const char* name : kManaged) {
    const std::string path = Join(resources_dir, name);
    std::string data;
    if (!ReadAll(path, &data)) {
      if (error) *error = std::string("missing resource: ") + name;
      return Manifest{};
    }
    FileEntry entry;
    entry.name = name;
    entry.size = data.size();
    entry.hash = Fnv1a(data.data(), data.size());
    manifest.files.push_back(entry);
    combined += name;
    combined += std::to_string(entry.size);
    combined += std::to_string(entry.hash);
  }
  manifest.digest = Fnv1a(combined.data(), combined.size());
  return manifest;
}

Status Prepare(const std::string& resources_dir,
               const std::string& user_dir) {
  Status status;
  status.resources_dir = resources_dir;
  status.user_dir = user_dir;
  DeployLock lock;
  if (!lock.acquired()) {
    status.detail = "deployment lock unavailable";
    return status;
  }
  std::string error;
  const Manifest manifest = BuildManifest(resources_dir, &error);
  if (manifest.empty()) {
    status.detail = error.empty() ? "empty resources" : error;
    diag::Event("deploy", "resources_missing");
    return status;
  }
  status.version = VersionString(manifest.digest);
  CreateDirectoryA(user_dir.c_str(), nullptr);

  const std::string marker = MarkerPath(user_dir);
  std::string current;
  if (ReadAll(marker, &current) && current == status.version + "\n") {
    status.ok = true;
    status.unchanged = true;
    diag::Event("deploy", "unchanged");
    return status;
  }

  // Back up the current set before replacing anything.
  const std::string backup = Join(user_dir, kBackupDir);
  RemoveTree(backup);
  bool had_previous = false;
  for (const char* name : kManaged) {
    const std::string src = Join(user_dir, name);
    if (!FileExists(src)) continue;
    CreateDirectoryA(backup.c_str(), nullptr);
    if (CopyFileA(src.c_str(), Join(backup, name).c_str(), FALSE)) {
      had_previous = true;
    }
  }
  if (had_previous && FileExists(marker))
    CopyFileA(marker.c_str(), Join(backup, kMarker).c_str(), FALSE);

  // Stage the new files, verify their hashes, then move them into place.
  const std::string stage = Join(user_dir, kStageDir);
  RemoveTree(stage);
  CreateDirectoryA(stage.c_str(), nullptr);
  bool copied_all = true;
  for (const FileEntry& entry : manifest.files) {
    const std::string src = Join(resources_dir, entry.name);
    const std::string staged = Join(stage, entry.name);
    if (!CopyFileA(src.c_str(), staged.c_str(), FALSE)) {
      status.detail = "copy failed: " + entry.name;
      copied_all = false;
      break;
    }
    std::string data;
    if (!ReadAll(staged, &data) || data.size() != entry.size ||
        Fnv1a(data.data(), data.size()) != entry.hash) {
      status.detail = "hash mismatch after copy: " + entry.name;
      copied_all = false;
      break;
    }
  }
  if (copied_all) {
    for (const FileEntry& entry : manifest.files) {
      if (!MoveFileExA(Join(stage, entry.name).c_str(),
                       Join(user_dir, entry.name).c_str(),
                       MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH)) {
        status.detail = "install failed: " + entry.name;
        copied_all = false;
        break;
      }
    }
  }
  RemoveTree(stage);
  if (!copied_all) {
    // Leave the marker untouched so the previous version stays authoritative.
    diag::Event("deploy", "prepare_failed");
    return status;
  }
  // The compiled output belongs to the previous source set; drop it so the
  // next initialization recompiles. Otherwise a stale build directory can mask
  // a broken schema file and defeat the last-known-good rollback.
  RemoveTree(Join(user_dir, kBuildDir));
  if (!FileExists(Join(user_dir, kPhrase))) {
    CopyFileA(Join(resources_dir, kPhrase).c_str(),
              Join(user_dir, kPhrase).c_str(), FALSE);
  }
  if (!WriteAll(Join(user_dir, kPending), status.version + "\n")) {
    status.detail = "cannot write pending marker";
    diag::Event("deploy", "marker_failed");
    return status;
  }
  status.ok = true;
  status.copied = true;
  diag::Event("deploy", "prepared");
  return status;
}

void Commit(const std::string& user_dir) {
  DeployLock lock;
  const std::string pending = Join(user_dir, kPending);
  if (FileExists(pending)) {
    MoveFileExA(pending.c_str(), MarkerPath(user_dir).c_str(),
                MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH);
  }
  RemoveTree(Join(user_dir, kBackupDir));
}

bool Rollback(const std::string& user_dir, std::string* detail) {
  DeployLock lock;
  const std::string backup = Join(user_dir, kBackupDir);
  const std::string marker = MarkerPath(user_dir);
  bool restored = false;
  if (DirExists(backup) && FileExists(Join(backup, kMarker))) {
    for (const char* name : kManaged) {
      const std::string src = Join(backup, name);
      if (FileExists(src))
        CopyFileA(src.c_str(), Join(user_dir, name).c_str(), FALSE);
    }
    CopyFileA(Join(backup, kMarker).c_str(), marker.c_str(), FALSE);
    restored = true;
  } else {
    for (const char* name : kManaged)
      DeleteFileA(Join(user_dir, name).c_str());
    DeleteFileA(marker.c_str());
  }
  DeleteFileA(Join(user_dir, kPending).c_str());
  RemoveTree(backup);
  RemoveTree(Join(user_dir, kStageDir));
  // Drop only AutoIME's own compile output so the retry rebuilds from the
  // restored sources; the Luna/stroke artifacts and shared dictionary tables
  // stay valid and are reused (recovery time budget).
  const std::string build = Join(user_dir, kBuildDir);
  const char* const stale[] = {
      "autoime_pinyin.schema.yaml", "autoime_pinyin.prism.bin",
      "autoime_pinyin.table.bin",   "autoime_terms.table.bin",
      "autoime_terms.prism.bin",    "autoime_terms.reverse.bin",
      "autoime_en.table.bin",       "autoime_en.reverse.bin",
  };
  for (const char* name : stale) DeleteFileA(Join(build, name).c_str());
  if (detail)
    *detail = restored ? "restored last known-good resources"
                       : "no previous resources; primary schema disabled";
  diag::Event("deploy", restored ? "rolled_back" : "rolled_back_empty");
  return restored;
}

bool CleanupLegacyLog(std::string* path) {
  char temp[MAX_PATH]{};
  if (GetTempPathA(MAX_PATH, temp) == 0) return false;
  const std::string legacy = Join(temp, "autoime-ime.log");
  if (!FileExists(legacy)) return false;
  if (path) *path = legacy;
  return DeleteFileA(legacy.c_str()) != 0;
}

}  // namespace deployment
