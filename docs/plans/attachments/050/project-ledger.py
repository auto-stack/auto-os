"""Surgical, locked offline projection for the reviewed PLAN-050 delta."""
import argparse
import copy
import hashlib
import json
import msvcrt
import time
from pathlib import Path

parser = argparse.ArgumentParser()
parser.add_argument('worktree', type=Path)
parser.add_argument('workspace', type=Path)
parser.add_argument('kind', choices=['os', 'lang'])
args = parser.parse_args()
out = Path(__file__).resolve().parent
root = args.worktree.resolve()
workspace = args.workspace.resolve()
ledger = root / '.autoos/specs.json'
decoder = json.JSONDecoder()
spans = {}

def scan(text, pos=0, path=()):
    while text[pos].isspace():
        pos += 1
    start = pos
    if text[pos] == '{':
        pos += 1
        while True:
            while text[pos].isspace():
                pos += 1
            if text[pos] == '}':
                pos += 1
                break
            key, end = decoder.raw_decode(text, pos)
            pos = end
            while text[pos].isspace():
                pos += 1
            assert text[pos] == ':'
            pos = scan(text, pos + 1, path + (key,))
            while text[pos].isspace():
                pos += 1
            if text[pos] == ',':
                pos += 1
    elif text[pos] == '[':
        pos += 1
        index = 0
        while True:
            while text[pos].isspace():
                pos += 1
            if text[pos] == ']':
                pos += 1
                break
            pos = scan(text, pos, path + (index,))
            index += 1
            while text[pos].isspace():
                pos += 1
            if text[pos] == ',':
                pos += 1
    else:
        _, pos = decoder.raw_decode(text, pos)
    spans[path] = (start, pos)
    return pos

def digest(data):
    return hashlib.sha256(data).hexdigest()

now = int(time.time())
delivery = '577594eb631b21a0816876863b7a09a30fd51f55' if args.kind == 'os' else 'ce055a5194649ad7c6251dbc46d0666bde08cfb6'
base_tags = ['PLAN-050:r4', 'source_commit:' + delivery,
             'archive:docs/plans/archive/050-app-directory-product-names.md']
updates = []

def add(section, item_id, title, file, content, test_file=None):
    source = workspace / file
    assert source.is_file(), source
    updates.append((section, item_id, title, file, content,
                    base_tags + ['source_sha256:' + digest(source.read_bytes())], test_file))

summary = 'PLAN-050:r4：物理产品目录使用 apps/auto-*，稳定 manifest ID、应用 pin、端口保持；相邻清单按真实路径提供 ID，Vue/VM 共用容器展开。24 个对应远端与固定提交可取回，branch=main；syslog 经用户确认保留内部组件。'
if args.kind == 'os':
    targets = ['apps/independent-repositories.md', 'shell/desktop-app-launch.md',
               'apps/syslog-app.md', 'apps/kanban.md', 'apps/tetris.md', 'apps/minesweeper.md']
    for index, target in enumerate(targets):
        file = 'docs/specs/' + target
        add('designs', 'P050-D' + str(index + 1), '应用产品路径与稳定 ID — ' + target, file, summary)
    add('tests', 'P050-TESTS', '产品目录迁移校验与固定提交保护',
        'docs/specs/apps/independent-repositories.md',
        'Python 5/5、24 gitlink 元数据、远端仓/固定提交 24/24、主检出 Explorer 43 文件/HEAD/clean/detached、Bash/PowerShell × Vue/iced dry-run 4/4；完整应用检出及 GUI 功能不属于该证据范围。',
        'scripts/tests/test_app_links.py')
    evidence_prefix = 'docs/plans/attachments/050/'
else:
    add('designs', 'OS050-REGISTRY', 'AutoOS 产品目录与稳定 ID',
        'docs/specs/auto-ui-compiler/project.md', summary)
    add('tests', 'OS050-TESTS', '产品路径注册与实际 Vue 生成回归',
        'docs/specs/auto-ui-compiler/project.md',
        '注册表 28/28、实际 Vue 生成 1/1、三轨发现一致性 1/1；旧 ID 拼目录负对照拒绝。完整日常门 5329/5331，两个已复现基线剪贴板失败经用户明确授权非阻塞，测试规则未改。',
        'crates/auto-ui-compiler/src/app_registry.rs')
    evidence_prefix = '../auto-os/docs/plans/attachments/050/'
add('reviews', 'P050-R4' if args.kind == 'os' else 'OS050-R4',
    'PLAN-050 r4 最终复审', evidence_prefix + 'final-review.md',
    '同一执行会话基于冻结源码、实际验收和规范哈希重建，final pass；非独立上下文。重放提交 range-diff 等价；源码、依赖、测试配置未改变。两项基线剪贴板失败的非阻塞处理由用户直接授权。')
add('reports', 'P050-MERGE' if args.kind == 'os' else 'OS050-MERGE',
    'PLAN-050 目录迁移合入记录', evidence_prefix + 'merge-receipt.json',
    '已线性合入 AutoOS 和 AutoUI 主线。24 个目录完成实际迁移，24 gitlink 原 pin 保持；Explorer 43 文件及 detached 状态保留。逐项复核与归档收尾状态见合入收据；归档计划 docs/plans/archive/050-app-directory-product-names.md。')

backup_dir = out / 'ledger-backups'
backup_dir.mkdir(exist_ok=True)
with ledger.open('r+b') as handle:
    msvcrt.locking(handle.fileno(), msvcrt.LK_NBLCK, 2147483647)
    before = handle.read()
    backup = backup_dir / ('auto-' + args.kind + '-' + digest(before)[:12] + '.json')
    if backup.exists():
        assert backup.read_bytes() == before
    else:
        backup.write_bytes(before)
    text = before.decode('utf-8')
    original = json.loads(text)
    expected = copy.deepcopy(original)
    assert {s['id'] for s in original['sections']} == {'goals', 'architecture', 'designs', 'tests', 'reviews', 'reports'}
    scan(text)
    edits = []
    append = {}
    changed = []
    for section_id, item_id, title, file, content, tags, test_file in updates:
        si = next(i for i, s in enumerate(expected['sections']) if s['id'] == section_id)
        section = expected['sections'][si]
        current = next((i for i in section['items'] if i.get('file') == file), None)
        if current is None:
            current = next((i for i in section['items'] if i['id'] == item_id), None)
        if current is not None:
            ii = section['items'].index(current)
            new = copy.deepcopy(current)
            if content not in new.get('content', ''):
                new['content'] = new.get('content', '') + '\n\n' + content
        else:
            new = {'id': item_id, 'title': title, 'content': content}
        defaults = {'status': 'verified', 'depends_on': [], 'related': [],
                    'priority': None, 'assignee': None, 'test_file': None,
                    'file': file, 'milestone': None, 'module': None, 'tags': [],
                    'created_at': now, 'modified_at': now, 'completed_at': now}
        for key, value in defaults.items():
            new.setdefault(key, value)
        new['modified_at'] = now
        new['status'] = 'verified'
        new['tags'] = list(dict.fromkeys(new['tags'] + tags))
        if test_file:
            assert (workspace / test_file).is_file()
            new['test_file'] = test_file
        rendered = json.dumps(new, ensure_ascii=False)
        if current is not None:
            section['items'][ii] = new
            edits.append((*spans[('sections', si, 'items', ii)], rendered))
        else:
            assert all(i['id'] != item_id for s in expected['sections'] for i in s['items'])
            section['items'].append(new)
            append.setdefault(si, []).append(rendered)
        changed.append({'section': section_id, 'id': new['id'], 'file': file})
    for si, values in append.items():
        start, end = spans[('sections', si, 'items')]
        populated = bool(original['sections'][si]['items'])
        addition = (',' if populated else '') + '\n' + ',\n'.join(values) + '\n'
        edits.append((end - 1, end - 1, addition))
    if 'version' in expected:
        expected['version'] += 1
        edits.append((*spans[('version',)], str(expected['version'])))
    after = text
    for start, end, replacement in sorted(edits, reverse=True):
        after = after[:start] + replacement + after[end:]
    assert json.loads(after) == expected
    # The only writes are item spans, insertions in affected item arrays, and version.
    # Reconstruct unchanged byte ranges independently to reject incidental formatting drift.
    cursor = 0
    pieces = []
    for start, end, replacement in sorted(edits):
        assert start >= cursor
        pieces.extend([text[cursor:start], replacement])
        cursor = end
    pieces.append(text[cursor:])
    assert ''.join(pieces) == after
    for section in expected['sections']:
        assert len({i['id'] for i in section['items']}) == len(section['items'])
    encoded = after.encode('utf-8')
    try:
        handle.seek(0)
        assert handle.read() == before
        handle.seek(0)
        handle.write(encoded)
        handle.truncate()
        handle.flush()
        handle.seek(0)
        assert handle.read() == encoded
    except BaseException:
        handle.seek(0)
        handle.write(before)
        handle.truncate()
        handle.flush()
        raise
    finally:
        handle.seek(0)
        msvcrt.locking(handle.fileno(), msvcrt.LK_UNLCK, 2147483647)
assert ledger.read_bytes() == encoded
receipt = {'channel': 'guarded offline projection, no store writer reachable',
           'workspace': str(workspace), 'worktree': str(root),
           'backup': str(backup), 'beforeSha256': digest(before),
           'afterSha256': digest(encoded), 'unrelatedBytesPreserved': True,
           'sixSectionsValidated': True, 'fileLock': 'Windows exclusive byte-range lock',
           'versionBefore': original.get('version'), 'versionAfter': expected.get('version'),
           'items': changed}
(out / ('ledger-' + args.kind + '-receipt.json')).write_text(json.dumps(receipt, ensure_ascii=False, indent=2), encoding='utf-8')
print(json.dumps(receipt, ensure_ascii=False))
