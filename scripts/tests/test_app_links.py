#!/usr/bin/env python3
"""Exercise the public app-link verifier with isolated uninitialized submodules."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

VERIFY = Path(__file__).resolve().parents[1] / "verify-app-links.py"
PIN = "1" * 40

class ProductPathVerification(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="autoos-app-paths-")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.git("init", "--quiet")
        self.git("update-index", "--add", "--cacheinfo", f"160000,{PIN},apps/auto-notes")
        self.modules = '[submodule "apps/015-notes"]\n path = apps/auto-notes\n url = https://github.com/auto-stack/auto-notes.git\n'
        (self.root / ".gitmodules").write_text(self.modules)
        self.manifest = {"apps": [
            {"id": "015-notes", "repo": "apps/auto-notes", "kind": "repo", "ports": [17818]},
            {"id": "039-syslog", "repo": "apps/auto-syslog", "kind": "local", "ports": [17800]},
        ]}
        self.write_manifest()
        reports = self.root / "docs/reports"
        reports.mkdir(parents=True)
        report = {"apps": [{"id": "015-notes", "name": "auto-notes", "submodulePath": "apps/015-notes", "commit": PIN}], "existingSubmodules": []}
        (reports / "v0.6-app-import.json").write_text(json.dumps(report))
        syslog = self.root / "apps/auto-syslog"
        syslog.mkdir(parents=True)
        (syslog / "pac.at").write_text('name: "syslog"\n')

    def git(self, *args):
        subprocess.run(["git", "-C", str(self.root), *args], check=True, capture_output=True, text=True)

    def write_manifest(self):
        (self.root / "apps.manifest").write_text(json.dumps(self.manifest))

    def verify(self, metadata=True):
        return subprocess.run([sys.executable, str(VERIFY), "--root", str(self.root), *(["--metadata-only"] if metadata else [])], capture_output=True, text=True)

    def test_current_paths_and_historical_report_are_accepted(self):
        result = self.verify()
        self.assertEqual(result.returncode, 0, result.stderr)
        receipt = json.loads(result.stdout)
        self.assertEqual(receipt["verificationMode"], "metadata")
        self.assertFalse(receipt["checkoutsVerified"])
        self.assertEqual(receipt["apps"][0]["path"], "apps/auto-notes")
        self.assertEqual(receipt["uninitialized"], ["apps/auto-notes"])

    def test_old_manifest_path_is_rejected(self):
        self.manifest["apps"][0]["repo"] = "apps/015-notes"
        self.write_manifest()
        result = self.verify()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Incorrect product root", result.stderr)

    def test_wrong_gitlink_is_rejected(self):
        self.git("update-index", "--cacheinfo", f"160000,{'2'*40},apps/auto-notes")
        result = self.verify()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Incorrect gitlink", result.stderr)

    def test_non_product_directory_is_rejected(self):
        self.git("update-index", "--force-remove", "apps/auto-notes")
        self.git("update-index", "--add", "--cacheinfo", f"160000,{PIN},apps/wrong")
        (self.root / ".gitmodules").write_text(self.modules.replace("path = apps/auto-notes", "path = apps/wrong"))
        result = self.verify()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Non-product submodule path", result.stderr)

    def test_full_mode_requires_initialized_checkout(self):
        result = self.verify(metadata=False)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Uninitialized app checkout", result.stderr)

if __name__ == "__main__":
    unittest.main()
