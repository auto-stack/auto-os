// AutoIME - resource deployment and recovery (PLAN-070 T-04).
//
// The shipped Rime resources live in one directory next to the installed DLL
// (<install>\resources). Deployment copies them into AutoIME's private user
// data dir, serializes concurrent cold starts with a named mutex, keeps a
// backup of the last known-good set, and rolls back when the new set fails to
// compile so the engine can fall back to the last working resources (and then
// to the Luna schema, and finally to keyboard pass-through).
#ifndef AUTOIME_DEPLOYMENT_H_
#define AUTOIME_DEPLOYMENT_H_

#include <cstdint>
#include <string>
#include <vector>

namespace deployment {

struct FileEntry {
  std::string name;
  uint64_t size = 0;
  uint64_t hash = 0;  // FNV-1a 64 over the file bytes
};

struct Manifest {
  std::vector<FileEntry> files;
  uint64_t digest = 0;  // combined content hash = resource version identity
  bool empty() const { return files.empty(); }
};

struct Status {
  bool ok = false;
  bool copied = false;
  bool unchanged = false;
  bool restored_backup = false;
  std::string resources_dir;
  std::string user_dir;
  std::string version;  // "r<hex digest>"
  std::string detail;
};

// Cross-process deployment gate: two hosts cold-starting at the same time
// serialize on this lock so only one compiles the dictionaries.
class DeployLock {
 public:
  DeployLock();
  ~DeployLock();
  DeployLock(const DeployLock&) = delete;
  DeployLock& operator=(const DeployLock&) = delete;
  bool acquired() const { return acquired_; }

 private:
  void* handle_ = nullptr;
  bool acquired_ = false;
};

// Resolution order: AUTOIME_RESOURCES_DIR -> <module_dir>\resources ->
// <module_dir>\..\resources -> <module_dir>\..\..\resources (dev build tree).
std::string ResolveResourcesDir(const std::string& module_dir);

// %LOCALAPPDATA%\AutoIME\rime (AUTOIME_RIME_USER_DATA_DIR overrides).
std::string DefaultUserDir();

// Builds the manifest of a resources directory (does not touch the user dir).
Manifest BuildManifest(const std::string& resources_dir, std::string* error);

// Copies/updates the managed resource files into the user dir. The previous
// set is backed up first; custom_phrase.txt is never overwritten.
Status Prepare(const std::string& resources_dir, const std::string& user_dir);

// Marks the prepared resources as last known-good and drops the backup.
void Commit(const std::string& user_dir);

// Restores the backed-up resources (or removes the failed set), drops the
// staged compile output and returns whether a previous set was restored.
bool Rollback(const std::string& user_dir, std::string* detail);

// Removes %TEMP%\autoime-ime.log (the PLAN-069 prototype wrote plaintext key
// traces there). Returns true when a file was found and deleted.
bool CleanupLegacyLog(std::string* path);

// Recursively deletes a directory tree (best effort).
bool RemoveTree(const std::string& path);

}  // namespace deployment

#endif  // AUTOIME_DEPLOYMENT_H_
