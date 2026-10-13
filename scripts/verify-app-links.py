#!/usr/bin/env python3
"""Verify submodule pins, manifest roots, portable dependencies and import provenance."""
import argparse, hashlib, json, re, subprocess
from pathlib import Path

parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--root',type=Path,default=Path(__file__).resolve().parents[1])
args=parser.parse_args()
root=args.root.resolve()
def git(repo,*argv):
    return subprocess.check_output(['git','-C',str(repo),'-c',f'safe.directory={repo.as_posix()}',
        '-c','core.excludesfile=',*argv],stderr=subprocess.PIPE)
data=json.loads((root/'docs/reports/v0.6-app-import.json').read_text(encoding='utf-8'))
manifest=json.loads((root/'apps.manifest').read_text(encoding='utf-8'))['apps']
assert len({a['id'] for a in manifest})==len(manifest),'Duplicate manifest ID'
registered={a['id']:a for a in manifest}
gitlinks={line.split()[3]:line.split()[1] for line in git(root,'ls-files','--stage').decode().splitlines()
          if line.startswith('160000')}
module_paths=dict(line.split(' ',1) for line in git(root,'config','-f','.gitmodules','--get-regexp',r'^submodule\..*\.path$').decode().splitlines())
assert set(module_paths.values())==set(gitlinks),'gitlinks/.gitmodules mismatch'
for key, path in module_paths.items():
    url=git(root,'config','-f','.gitmodules','--get',key[:-4]+'url').decode().strip()
    assert re.fullmatch(r'https://github\.com/auto-stack/[a-z0-9-]+\.git',url),f'Nonportable submodule URL {path}: {url}'
used_ports={}
for app in manifest:
    for port in app.get('ports',[]):
        assert port not in used_ports, f'Port collision {port}: {used_ports.get(port)}/{app["id"]}'
        used_ports[port]=app['id']
results=[]
for app in data['apps']:
    path=app['submodulePath']; checkout=root/path
    assert gitlinks[path]==app['commit'],f'Incorrect gitlink {path}'
    assert git(checkout,'rev-parse','HEAD').decode().strip()==app['commit'],f'Uninitialized/mismatched {path}'
    assert not git(checkout,'status','--porcelain').strip(),f'Dirty submodule {path}'
    assert registered[app['id']]['repo']==path and registered[app['id']]['kind']=='repo'
    assert (checkout/'pac.at').is_file() and (checkout/'src/front/app.at').is_file()
    pac=(checkout/'pac.at').read_text(encoding='utf-8')
    assert re.search(r'^front_port:\s*'+str(app['frontPort'])+r'\s*$',pac,re.M)
    assert re.search(r'^desktop:\s*"true"',pac,re.M)
    for relative in re.findall(r'^\s*path:\s*"([^"]+)"',pac,re.M):
        assert (checkout/relative).is_dir(),f'Unresolved dep {path}: {relative}'
    for key in ['photo_root','media_root']:
        match=re.search(r'^'+key+r':\s*"([^"]*)"',pac,re.M)
        assert not match or not match[1],f'Developer-specific media root {path}'
    provenance=json.loads((checkout/'SOURCE-IMPORT.json').read_text(encoding='utf-8'))
    file_count=0
    for source in provenance['sources']:
        for file in source['files']:
            content=git(checkout,'show',f'{app["sourceCommit"]}:{file["path"]}')
            assert hashlib.sha256(content).hexdigest()==file['sha256'],f'Source hash mismatch {path}:{file["path"]}'
            file_count+=1
    results.append({'repository':app['name'],'path':path,'commit':app['commit'],'sourceFilesVerified':file_count})
for app in data.get('existingSubmodules',[]):
    assert gitlinks[app['path']]==app['commit']
    assert git(root/app['path'],'rev-parse','HEAD').decode().strip()==app['commit']
    assert not git(root/app['path'],'status','--porcelain').strip()
assert (root/'apps/039-syslog/pac.at').exists(),'System log must remain internal'
assert 'auto-system-log' not in (root/'.gitmodules').read_text(encoding='utf-8')
print(json.dumps({'newProductsVerified':len(results),'totalAppSubmodules':len(gitlinks),
    'existingAppsVerified':len(data.get('existingSubmodules',[])),'result':'pass','apps':results},
    ensure_ascii=False,indent=2))
