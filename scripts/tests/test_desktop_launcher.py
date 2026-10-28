"""Wrapper process-boundary tests. Controlled builds/launches do not prove GUI parity."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

SCRIPTS = Path(__file__).resolve().parents[1]
BASH = 'C:/Program Files/Git/bin/bash.exe' if os.name == 'nt' else shutil.which('bash')
PWSH = shutil.which('pwsh')
PS_HARNESS = r'''
. $env:DESKTOP_SCRIPT
if ($env:NATIVE_CARGO -ne '1') {
    function Invoke-DesktopCargo([string]$Root, [string]$Target, [string[]]$ArgumentList) {
        & $env:DESKTOP_PYTHON $env:DESKTOP_FIXTURE cargo $Root $Target @ArgumentList
        if ($LASTEXITCODE -ne 0) { Throw-DesktopFailure 'controlled build failure' $LASTEXITCODE }
    }
}
function Invoke-DesktopProgram([string]$Executable, [string[]]$ArgumentList) {
    & $env:DESKTOP_PYTHON $env:DESKTOP_FIXTURE launch $Executable @ArgumentList
    if ($LASTEXITCODE -ne 0) { Throw-DesktopFailure 'controlled launch failure' $LASTEXITCODE }
}
$old = @{}; $cwd = (Get-Location).Path
$keys = @('AUTO_OS_ROOT','AUTO_LANG_ROOT','AUTO_UI_ROOT','AUTO_MAN_ROOT','AUTO_UI_TOOL_BIN','AUTO_DESKTOP_APPS','AUTO_DESKTOP_APPS_EXTRA','AUTO_DESKTOP_RQHOST','AUTO_MPV_LIB','CARGO_TARGET_DIR')
foreach ($key in $keys) { $old[$key] = [Environment]::GetEnvironmentVariable($key,'Process') }
$code = 0
try { $options = ConvertFrom-Json -AsHashtable $env:DESKTOP_OPTIONS; Invoke-DesktopMain @options }
catch { $code = if ($_.Exception.Data.Contains('DesktopExitCode')) { [int]$_.Exception.Data['DesktopExitCode'] } else { 1 }; Write-Host $_ }
foreach ($key in $keys) { if ($old[$key] -cne [Environment]::GetEnvironmentVariable($key,'Process')) { Write-Host "not restored: $key"; exit 91 } }
if ($cwd -ne (Get-Location).Path) { exit 92 }
exit $code
'''
BASH_HARNESS = r'''
source "$DESKTOP_SCRIPT"
if [[ "${NATIVE_CARGO:-}" != 1 ]]; then desktop_cargo() { "$DESKTOP_PYTHON" "$DESKTOP_FIXTURE" cargo "$@"; }; fi
desktop_program() { "$DESKTOP_PYTHON" "$DESKTOP_FIXTURE" launch "$@"; }
old_dir="$PWD"; old_tool="${AUTO_UI_TOOL_BIN:-}"
set +e
desktop_main "$@"
code=$?
[[ "$PWD" = "$old_dir" && "${AUTO_UI_TOOL_BIN:-}" = "$old_tool" ]] || exit 91
exit "$code"
'''
FIXTURE = r'''
import json, os, sys
from pathlib import Path
kind=sys.argv[1]
record={'kind':kind,'cwd':os.getcwd(),'args':sys.argv[2:],'tool':os.environ.get('AUTO_UI_TOOL_BIN'),'os':os.environ.get('AUTO_OS_ROOT'),'mpv':os.environ.get('AUTO_MPV_LIB'),'extra':os.environ.get('AUTO_DESKTOP_APPS_EXTRA')}
with open(os.environ['TRACE'],'a',encoding='utf-8') as f:f.write(json.dumps(record,ensure_ascii=False)+'\n')
if kind=='cargo':
    root,target=sys.argv[2:4];args=sys.argv[4:]
    if Path(root).name==os.environ.get('FAIL_REPO'):sys.exit(17)
    if os.environ.get('NO_ARTIFACT')=='1':sys.exit(0)
    out=Path(target)/'debug';suffix='.exe' if os.name=='nt' else ''
    if '--example' in args:out=out/'examples';name=args[args.index('--example')+1]
    else:name=args[args.index('--bin')+1]
    out.mkdir(parents=True,exist_ok=True);(out/(name+suffix)).write_text('controlled artifact')
else:sys.exit(int(os.environ.get('LAUNCH_EXIT','0')))
'''

class DesktopLauncher(unittest.TestCase):
    def setUp(self):
        self.temp=tempfile.TemporaryDirectory(prefix='desktop-launch-');self.addCleanup(self.temp.cleanup)
        self.root=Path(self.temp.name)/'space 中文';self.root.mkdir()
        self.osroot=self.root/'auto-os';(self.osroot/'apps').mkdir(parents=True)
        for repo,probe in [('auto-lang','crates/auto-lang/Cargo.toml'),('auto-ui','crates/auto-ui-tools/Cargo.toml'),('auto-man','crates/auto/Cargo.toml')]:
            p=self.root/repo/probe;p.parent.mkdir(parents=True);p.write_text('[package]\n')
        (self.root/'auto-lang/examples/desktop-host').mkdir(parents=True)
        self.trace=self.root/'trace.jsonl';self.fixture=self.root/'fixture.py';self.fixture.write_text(FIXTURE)
        self.ps=self.root/'harness.ps1';self.ps.write_text(PS_HARNESS);self.sh=self.root/'harness.sh';self.sh.write_text(BASH_HARNESS)
        self.env=os.environ.copy()
        for k in ['AUTO_LANG_ROOT','AUTO_UI_ROOT','AUTO_MAN_ROOT','AUTO_DESKTOP_BUILD_ROOT','CARGO_TARGET_DIR']:self.env.pop(k,None)
        self.env.update(DESKTOP_OS_ROOT=str(self.osroot),TRACE=str(self.trace),DESKTOP_FIXTURE=str(self.fixture),DESKTOP_PYTHON=sys.executable,AUTO_UI_TOOL_BIN='previous-tool',AUTO_DESKTOP_APPS_EXTRA='previous-extra',AUTO_MPV_LIB='explicit-mpv',CARGO_TARGET_DIR='previous-target')
        self.suffix='.exe' if os.name=='nt' else ''

    def run_launcher(self,engine,**options):
        self.trace.unlink(missing_ok=True);env=self.env.copy()
        if engine=='ps':
            if not PWSH:self.skipTest('pwsh unavailable')
            env['DESKTOP_SCRIPT']=str(SCRIPTS/'desktop.ps1');env['DESKTOP_OPTIONS']=json.dumps(options)
            command=[PWSH,'-NoProfile','-File',str(self.ps)]
        else:
            env['DESKTOP_SCRIPT']=str(SCRIPTS/'desktop.sh');command=[BASH,str(self.sh),options.get('Track','vue')]
            mapping={'Fullscreen':'--fullscreen','DryRun':'--dry-run','LangRoot':'--lang-root','UiRoot':'--ui-root','ManRoot':'--man-root','BuildRoot':'--build-root','CandidateDir':'--candidate-dir'}
            for key,value in options.items():
                if key=='Track':continue
                command.append(mapping[key])
                if key not in ('Fullscreen','DryRun'):command.append(str(value))
        result=subprocess.run(command,env=env,capture_output=True,text=True,encoding='utf-8',errors='replace',timeout=25)
        events=[json.loads(line) for line in self.trace.read_text(encoding='utf-8').splitlines()] if self.trace.exists() else []
        return result,events

    def artifacts(self,directory,host=True):
        directory.mkdir(parents=True,exist_ok=True)
        for name in ['auto','auto-ui-tool']+(['ui_desktop'] if host else []):(directory/(name+self.suffix)).write_text('controlled artifact')

    def test_dry_run_has_no_build_or_write(self):
        for engine in ('ps','bash'):
            with self.subTest(engine=engine):
                r,e=self.run_launcher(engine,Track='iced',DryRun=True,Fullscreen=True)
                self.assertEqual(r.returncode,0,r.stdout+r.stderr);self.assertEqual(e,[])
                self.assertFalse((self.osroot/'.auto').exists());self.assertIn('auto-ui-tools',r.stdout);self.assertIn('--fullscreen',r.stdout)

    def test_source_build_uses_three_owners_and_one_target(self):
        for engine in ('ps','bash'):
            with self.subTest(engine=engine):
                r,e=self.run_launcher(engine,Track='iced',Fullscreen=True);self.assertEqual(r.returncode,0,r.stdout+r.stderr)
                builds=e[:-1];self.assertEqual([Path(x['args'][0]).name for x in builds],['auto-man','auto-ui','auto-lang'])
                self.assertEqual(len({Path(x['args'][1]) for x in builds}),1)
                launch=e[-1];self.assertEqual(launch['kind'],'launch');self.assertEqual(Path(launch['args'][0]).name,'ui_desktop'+self.suffix)
                self.assertEqual(Path(launch['cwd']),self.osroot);self.assertIn('--fullscreen',launch['args']);self.assertEqual(launch['mpv'],'explicit-mpv')
                self.assertEqual(Path(launch['tool']).parent,Path(builds[0]['args'][1])/'debug')
                self.assertTrue((Path(launch['args'][0]).parent.parent/('auto'+self.suffix)).is_file())

    def test_failed_build_does_not_launch_stale_artifacts(self):
        target=self.osroot/'.auto/desktop-runtime/target/debug';self.artifacts(target)
        (target/'examples').mkdir();(target/'examples'/('ui_desktop'+self.suffix)).write_text('old')
        for engine in ('ps','bash'):
            for repo in ('auto-man','auto-ui','auto-lang'):
                with self.subTest(engine=engine,repo=repo):
                    self.env['FAIL_REPO']=repo;r,e=self.run_launcher(engine,Track='iced')
                    self.assertEqual(r.returncode,17,r.stdout+r.stderr);self.assertFalse(any(x['kind']=='launch' for x in e))
        self.env.pop('FAIL_REPO')

    def test_missing_artifact_after_success_is_error(self):
        self.env['NO_ARTIFACT']='1'
        for engine in ('ps','bash'):
            with self.subTest(engine=engine):
                r,e=self.run_launcher(engine,Track='iced');self.assertNotEqual(r.returncode,0);self.assertFalse(any(x['kind']=='launch' for x in e))

    def test_bad_explicit_root_and_non_sibling_group(self):
        p=self.root/'other/auto-ui/crates/auto-ui-tools/Cargo.toml';p.parent.mkdir(parents=True);p.write_text('x')
        renamed=self.root/'renamed-ui/crates/auto-ui-tools/Cargo.toml';renamed.parent.mkdir(parents=True);renamed.write_text('x')
        for engine in ('ps','bash'):
            for opts in ({'UiRoot':str(self.root/'absent')},{'UiRoot':str(self.root/'other/auto-ui')},{'UiRoot':str(self.root/'renamed-ui')}):
                with self.subTest(engine=engine,opts=opts):
                    r,e=self.run_launcher(engine,Track='iced',**opts);self.assertNotEqual(r.returncode,0);self.assertEqual(e,[])

    def test_candidate_iced_needs_no_source_or_cargo(self):
        bundle=self.root/'candidate bin';self.artifacts(bundle);self.env['AUTO_LANG_ROOT']='invalid ignored source'
        for engine in ('ps','bash'):
            with self.subTest(engine=engine):
                r,e=self.run_launcher(engine,Track='iced',CandidateDir=str(bundle));self.assertEqual(r.returncode,0,r.stdout+r.stderr)
                self.assertEqual([x['kind'] for x in e],['launch']);self.assertEqual(Path(e[0]['tool']),bundle/('auto-ui-tool'+self.suffix))

    def test_candidate_missing_tool_has_no_fallback(self):
        bundle=self.root/'candidate';self.artifacts(bundle);(bundle/('auto-ui-tool'+self.suffix)).unlink()
        for engine in ('ps','bash'):
            with self.subTest(engine=engine):
                r,e=self.run_launcher(engine,Track='iced',CandidateDir=str(bundle));self.assertNotEqual(r.returncode,0);self.assertEqual(e,[])

    def test_vue_project_and_environment(self):
        for engine in ('ps','bash'):
            with self.subTest(engine=engine):
                r,e=self.run_launcher(engine);self.assertEqual(r.returncode,0,r.stdout+r.stderr);self.assertEqual(len(e),3)
                self.assertEqual(e[-1]['args'][1:],['run','--desktop']);self.assertEqual(Path(e[-1]['cwd']),self.root/'auto-lang/examples/desktop-host');self.assertIsNone(e[-1]['extra'])

    def test_launch_failure_code(self):
        bundle=self.root/'candidate';self.artifacts(bundle);self.env['LAUNCH_EXIT']='31'
        for engine in ('ps','bash'):
            with self.subTest(engine=engine):
                r,e=self.run_launcher(engine,Track='iced',CandidateDir=str(bundle));self.assertEqual(r.returncode,31,r.stdout+r.stderr)

    def test_conflicting_candidate_and_build_root(self):
        for engine in ('ps','bash'):
            with self.subTest(engine=engine):
                r,e=self.run_launcher(engine,Track='iced',CandidateDir='candidate',BuildRoot='build');self.assertNotEqual(r.returncode,0);self.assertEqual(e,[])

    def test_public_entries_dry_run_and_native_failure(self):
        native=self.root/'public native';native.mkdir()
        (native/'cargo.cmd').write_text('@echo off\r\nexit /b 23\r\n')
        (native/'cargo').write_text('#!/usr/bin/env bash\nexit 23\n');(native/'cargo').chmod(0o755)
        env=self.env.copy();env['PATH']=str(native)+os.pathsep+env['PATH']
        for engine in ('ps','bash'):
            base=[PWSH,'-NoProfile','-File',str(SCRIPTS/'desktop.ps1'),'-Track','iced'] if engine=='ps' else [BASH,str(SCRIPTS/'desktop.sh'),'iced']
            for dry in (True,False):
                with self.subTest(engine=engine,dry=dry):
                    command=base+(['-DryRun'] if engine=='ps' and dry else ['--dry-run'] if dry else [])
                    r=subprocess.run(command,env=env,capture_output=True,text=True,encoding='utf-8',errors='replace',timeout=15)
                    self.assertEqual(r.returncode,0 if dry else 23,r.stdout+r.stderr)
                    self.assertFalse((self.osroot/'.auto').exists());self.assertFalse(self.trace.exists())

    def test_candidate_dry_run_and_explicit_root_precedence(self):
        self.env['AUTO_UI_ROOT']='invalid lower priority root'
        for engine in ('ps','bash'):
            with self.subTest(engine=engine):
                r,e=self.run_launcher(engine,Track='iced',UiRoot=str(self.root/'auto-ui'),DryRun=True)
                self.assertEqual(r.returncode,0,r.stdout+r.stderr);self.assertEqual(e,[])
                r,e=self.run_launcher(engine,Track='iced',CandidateDir=str(self.root/'not built yet'),DryRun=True)
                self.assertEqual(r.returncode,0,r.stdout+r.stderr);self.assertEqual(e,[])
                self.assertFalse((self.root/'not built yet').exists())

    def test_native_cargo_failure_propagates(self):
        native=self.root/'native commands';native.mkdir();(native/'cargo.cmd').write_text('@echo off\r\nexit /b 23\r\n')
        (native/'cargo').write_text('#!/usr/bin/env bash\nexit 23\n');(native/'cargo').chmod(0o755)
        self.env['PATH']=str(native)+os.pathsep+self.env['PATH'];self.env['NATIVE_CARGO']='1'
        for engine in ('ps','bash'):
            with self.subTest(engine=engine):
                r,e=self.run_launcher(engine,Track='iced');self.assertEqual(r.returncode,23,r.stdout+r.stderr);self.assertEqual(e,[])

if __name__=='__main__':unittest.main()
