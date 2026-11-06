#!/usr/bin/env python3
"""Reconcile a real desktop run against the selected candidate and disk oracles.

This gate consumes a run packet collected during actual UI tasks. It never turns
an app's standalone pass or an outcome flag into desktop evidence. A packet must
bind all eight apps, the immutable SDK, rendered create/reopen/next observations,
and independent persisted results. It does not launch or prewrite product APIs.
Raw packets belong outside Git; keep the durable commands/counts in the Plan.
"""
import argparse
import hashlib
import json
import math
from pathlib import Path
import re
import sqlite3
import subprocess
import sys
import xml.etree.ElementTree as ET

TARGETS = {
    'notes': '015-notes', 'todo': '013-todo', 'calendar': '016-calendar',
    'paint': '031-paint', 'database': '026-database', 'chat': '017-chat',
    'blog': '023-realworld', 'charts': '024-charts',
}


def require(condition, reason):
    if not condition:
        raise ValueError(reason)


def read_json(path):
    return json.loads(Path(path).read_text(encoding='utf-8-sig'))


def digest(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def contains(value, expected):
    if value == expected:
        return True
    if isinstance(value, dict):
        return any(contains(v, expected) for v in value.values())
    if isinstance(value, list):
        return any(contains(v, expected) for v in value)
    return isinstance(value, str) and isinstance(expected, str) and expected in value


def git(root, *args):
    return subprocess.check_output(
        ['git', '-C', str(root), *args], stderr=subprocess.PIPE,
        text=True, encoding='utf-8').strip()


def artifact(record, png=False):
    p = Path(record['path']).resolve(strict=True)
    require(p.is_file(), f'Not a file: {p}')
    require(digest(p) == record['sha256'], f'Artifact changed: {p}')
    if png:
        head = p.read_bytes()[:24]
        require(head[:8] == b'\x89PNG\r\n\x1a\n', f'Not a PNG: {p}')
        require(int.from_bytes(head[16:20], 'big') > 100 and
                int.from_bytes(head[20:24], 'big') > 100, f'Empty frame: {p}')
    return p


def sql(path, query, args=()):
    uri = Path(path).resolve(strict=True).as_uri() + '?mode=ro'
    with sqlite3.connect(uri, uri=True) as db:
        db.execute('PRAGMA query_only=ON')
        return [list(row) for row in db.execute(query, args)]


def check_identity(root, packet):
    lock = read_json(artifact(packet['candidate_lock']))
    manifest = read_json(root / 'apps.manifest')['apps']
    registered = {a['id']: a for a in manifest}
    require(len(registered) == len(manifest), 'Duplicate manifest ID')
    require(set(registered) == set(packet['registry_ids']), 'Registry set drift')
    links = {line.split()[3]: line.split()[2]
             for line in git(root, 'ls-tree', '-r', 'HEAD', 'apps/').splitlines()
             if line.startswith('160000 ')}
    require(links == packet['gitlinks'], 'Candidate gitlink set drift')
    selected = {a['id']: a for a in lock['apps']}
    require(set(selected) == set(TARGETS.values()), 'Lock must select exactly eight apps')
    for name, app_id in TARGETS.items():
        a = selected[app_id]
        checkout = root / a['repo']
        require(links.get(a['repo']) == a['pin'], f'{name}: gitlink differs from lock')
        require(git(checkout, 'rev-parse', 'HEAD') == a['pin'], f'{name}: checkout pin drift')
        require(not git(checkout, 'diff', '--name-only', 'HEAD', '--',
                        'pac.at', 'src', 'docs/specs'), f'{name}: modified product source')
        require(digest(checkout / 'pac.at') == a['pac_sha256'], f'{name}: pac drift')
        actual = registered[app_id]
        for key in ('ports', 'launch'):
            require(actual.get(key) == a.get(key), f'{name}: {key} declaration drift')
        require(actual.get('native_http', False) == a.get('native_http', False),
                f'{name}: backend profile drift')
    require(set(packet['sdk']) == {'auto.exe', 'auto-ui-tool.exe', 'ui-desktop.exe'},
            'Exactly three immutable SDK programs required')
    for name, record in packet['sdk'].items():
        require(artifact(record).name == name, 'SDK program name differs from its binding')
    # Existing health gate checks every registered product, including non-targets.
    health = subprocess.run(
        ['pwsh', '-NoProfile', '-File', str(root/'scripts/check-app-submodules.ps1'),
         '-OsRoot', str(root)], capture_output=True, text=True, encoding='utf-8')
    require(health.returncode == 0, 'Submodule health failed: ' + health.stdout + health.stderr)


def check_frames(case):
    require(case.get('api_prewrites') is False, 'UI tasks must not be API/DB prewrites')
    require(case.get('status') == 'completed', 'Required task is pending/skipped/failed')
    require('next' in case['frames'], 'Next-operation rendered frame required')
    for record in case['frames'].values():
        artifact(record, png=True)
    # Reopen may be a separate frame or the original VM/DOM observation made by
    # the task runner. Do not invent an extra screenshot quota for old runs.
    artifact(case['reopen_observation'])
    # Frozen task observations and actual frame bytes are checked separately from
    # the current persisted data. Inspecting the frames remains a review duty;
    # this evidence gate does not claim to replay the original UI gestures.
    return read_json(artifact(case['task_observation']))


def check_notes(c, ui):
    e = c['expected']
    rows = sql(c['database'], 'SELECT legacy_id,title,body,revision FROM notes WHERE legacy_id=?', [e['id']])
    require(len(rows) == 1 and rows[0][1:3] == [e['title'], e['body']] and
            rows[0][3] >= e['minimum_revision'], 'Notes persisted content/revision differs')
    require(contains(ui, e['title']) and contains(ui, e['body']),
            'Notes task observation lacks intended content')


def check_todo(c, ui):
    e = c['expected']
    require(sql(c['database'], 'SELECT id,text,done FROM todos WHERE id=?', [e['id']]) ==
            [[e['id'], e['text'], e['done']]], 'Todo persisted result differs')
    require(contains(ui, e['text']), 'Todo task observation lacks intended text')


def check_calendar(c, ui):
    root = Path(c['data_root'])
    manifest = read_json(root/'store/manifest.json')
    identity = read_json(root/'store/identity.json')
    e = c['expected']
    require(identity['generation'] == e['generation'] and
            identity['rev_high'] >= e['minimum_revision'], 'Calendar recovery identity regressed')
    active = {a['id']: a for a in manifest['events']}
    require(not set(e['deleted_ids']) & set(active), 'Deleted Calendar event reappeared')
    require(e['next_id'] in active, 'Calendar next event absent')
    event = read_json(root/'store/events'/active[e['next_id']]['file'])
    require(event['title'] == e['next_title'], 'Calendar next event content differs')
    require(contains(ui, e['next_title']), 'Calendar task result absent')


def check_database(c, ui):
    for fixture in c['fixtures']:
        p = artifact(fixture)
        require(sql(p, fixture['query']) == fixture['expected_rows'], 'Database query result differs')
        for table in fixture['forbidden_tables']:
            require(not sql(p, 'SELECT name FROM sqlite_master WHERE name=?', [table]),
                    'Readonly Database probe wrote a table')
    require(contains(ui, c['rejection_marker']), 'Readonly rejection observation absent')


def check_chat(c, ui):
    rows = sql(c['database'], 'SELECT conversation_id,sender_id,text,client_msg_id FROM messages ORDER BY id')
    expected = c['expected_messages']
    require(len(expected) >= 3, 'Chat needs A/B exchange and next send')
    for row in expected:
        require([r for r in rows if r[3] == row[3]] == [row], 'Chat message identity/content mismatch')
    require(len({r[1] for r in expected}) >= 2 and len({r[0] for r in expected}) == 1,
            'Chat requires two senders in the same conversation')
    require(len({r[3] for r in expected}) == len(expected), 'Chat replay duplicated message key')
    require(all(contains(ui, r[2]) for r in expected), 'Chat exchange observation missing')


def check_blog(c, ui):
    e = c['expected']
    root = Path(c['work_root'])
    meta = read_json(root/c['meta_file'])
    body = (root/c['body_file']).read_text(encoding='utf-8')
    require(meta['title'] == e['title'] and body == e['body'] and
            meta['revision'] >= e['minimum_revision'], 'Blog saved author work differs')
    require(contains(ui, e['title']) and contains(ui, e['body']),
            'Blog author task observation missing')


def check_paint(c, ui):
    pixels = c['expected_pixels']
    require(len(pixels) == 256, 'Paint requires full 16x16 oracle')
    require(any(color.lower() != '#ffffff' for color in pixels), 'Paint task contains no drawn pixel')
    tree = ET.parse(artifact(c['svg'])).getroot()
    expanded = ['#ffffff'] * 256
    for rect in tree.iter('{http://www.w3.org/2000/svg}rect'):
        x, y = int(rect.get('x', 0)), int(rect.get('y', 0))
        width, height = int(rect.get('width', 0)), int(rect.get('height', 0))
        for row in range(y, y+height):
            for col in range(x, x+width):
                require(0 <= row < 16 and 0 <= col < 16, 'Paint SVG rectangle outside grid')
                expanded[row*16+col] = rect.get('fill')
    require(expanded == pixels, 'Paint SVG differs from all expected pixels')
    colors = c['rendered_colors']
    observed = []
    for color in colors:
        if re.fullmatch(r'#[0-9a-fA-F]{6}', color):
            observed.append(color.lower())
        else:
            match = re.fullmatch(r'rgb\((\d+),\s*(\d+),\s*(\d+)\)', color)
            require(match is not None, 'Unrecognized rendered Paint color')
            observed.append('#' + ''.join(f'{int(v):02x}' for v in match.groups()))
    require(observed == pixels and contains(ui, colors),
            'Paint rendered pixel observation differs')
    if 'storage_file' in c:
        storage = read_json(c['storage_file'])
        values = [v for k, v in storage.items() if 'paint.project.v2' in k]
    else:
        values = [c['local_storage_value']]
        require(contains(ui, values[0]), 'Paint browser storage observation absent')
    require(len(values) == 1 and values[0].split('|')[3:] == pixels,
            'Paint persisted pixel grid differs')


def check_charts(c, ui):
    project = read_json(c['project_file'])
    require(json.loads(project['rows_frag']) == c['expected_rows'] and
            project['index_col'] == c['index_col'] and project['value_cols'] == c['value_cols'],
            'Charts saved rows/mapping differ')
    root = ET.parse(artifact(c['svg'])).getroot()
    bars = [e for e in root.iter('{http://www.w3.org/2000/svg}rect')
            if e.get('fill') == '#2563eb' and float(e.get('height', 0)) > 10]
    values = c['values']
    require(len(bars) == len(values) == 3, 'Charts requires three actual bars')
    heights = [float(b.get('height')) for b in bars]
    baseline = [float(b.get('y'))+float(b.get('height')) for b in bars]
    require(max(baseline)-min(baseline) < .02, 'Charts bars lack a shared zero baseline')
    for actual in (heights,):
        require(all(math.isclose(actual[i]/actual[1], values[i]/values[1], abs_tol=.001)
                    for i in range(3)), 'Charts decimal bar geometry differs')
    if 'rendered_plot' in c:
        # Measure the real iced frame independently of the exported SVG.
        try:
            from PIL import Image
        except ImportError as error:
            raise ValueError('Pillow is required to measure native Charts frame') from error
        with Image.open(artifact(c['rendered_plot'], png=True)) as frame:
            image = frame.convert('RGB')
            width, _ = image.size
            pixels = image.get_flattened_data() if hasattr(image, 'get_flattened_data') else image.getdata()
            points = {(i % width, i // width) for i, color in enumerate(pixels) if color == (37, 99, 235)}
        boxes = []
        while points:
            queue = [points.pop()]
            xs, ys = [], []
            while queue:
                x, y = queue.pop()
                xs.append(x); ys.append(y)
                for neighbor in ((x-1,y), (x+1,y), (x,y-1), (x,y+1)):
                    if neighbor in points:
                        points.remove(neighbor); queue.append(neighbor)
            left, top, right, bottom = min(xs), min(ys), max(xs)+1, max(ys)+1
            w, h = right-left, bottom-top
            if w >= 30 and h >= 20 and len(xs)/(w*h) > .9:
                boxes.append((left, top, right, bottom))
        boxes.sort()
        require(len(boxes) == 3, 'Native Charts frame lacks three solid blue bars')
        raster = [b[3]-b[1] for b in boxes]
        require(max(b[3] for b in boxes)-min(b[3] for b in boxes) <= 1,
                'Native Charts rendered zero baseline differs')
        # Each rasterized edge may round by one physical pixel. This does not
        # loosen the independent floating-point SVG ratio check above.
        require(all(abs(raster[i]-raster[1]*values[i]/values[1]) <= 2 for i in range(3)),
                'Native Charts rendered decimal bar heights differ')
        require(contains(ui, project['name']), 'Charts reopened project observation missing')
    else:
        actual = c['rendered_heights']
        require(len(actual) == 3 and all(math.isclose(actual[i]/actual[1], values[i]/values[1], abs_tol=.001)
                    for i in range(3)), 'Charts DOM geometry differs')
        require(contains(ui, actual) or contains(ui, c['rendered_geometry']),
                'Charts DOM geometry observation missing')


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--profile', choices=['vue', 'iced'], required=True)
    ap.add_argument('--os-root', type=Path, required=True)
    ap.add_argument('--run', type=Path, required=True, help='External real-run packet; not a list of pass flags')
    args = ap.parse_args()
    packet = read_json(args.run)
    require(packet['schema'] == 'basic-app-assembly-run/1' and packet['profile'] == args.profile,
            'Run schema/profile mismatch')
    require(set(packet['cases']) == set(TARGETS), 'Exactly eight required desktop scenarios must be present')
    failures = []
    try:
        check_identity(args.os_root.resolve(strict=True), packet)
    except (ValueError, KeyError, OSError, subprocess.SubprocessError) as e:
        failures.append(f'candidate: {e}')
    executed = 0
    for name, case in packet['cases'].items():
        try:
            ui = check_frames(case)
            globals()['check_'+name](case, ui)
            executed += 1
            print('PASS ' + name)
        except (ValueError, KeyError, OSError, sqlite3.Error, ET.ParseError) as e:
            failures.append(f'{name}: {e}')
    for failure in failures:
        print('FAIL ' + failure, file=sys.stderr)
    print(json.dumps({'profile': args.profile, 'required': 8, 'passed': executed,
                      'failed': len(failures), 'gate_passed': executed == 8 and not failures}))
    return 0 if executed == 8 and not failures else 1


if __name__ == '__main__':
    try:
        sys.exit(main())
    except (ValueError, KeyError, OSError, TypeError) as error:
        print('FAIL required input: ' + str(error), file=sys.stderr)
        sys.exit(1)
