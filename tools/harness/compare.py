"""Screenshot regression: run one script with two builds and diff every shot.

usage: python3 tools/harness/compare.py <script.txt> <bin_a> <bin_b> [work_dir] [lib_dir]
  work_dir default: /tmp/fdl-compare   (a/, b/ and diff-<shot>.png land here)
Exit status 1 when any shot differs. Needs Pillow. A pure refactor should
report every shot identical. The caret does not blink (FDL_NOBLINK); scripts
should park the pointer first, since the real mouse position shows as hover.
"""
import os, subprocess, sys
from PIL import Image, ImageChops
here = os.path.dirname(os.path.abspath(__file__))
script, bins = sys.argv[1], sys.argv[2:4]
work = os.path.abspath(sys.argv[4]) if len(sys.argv) > 4 else '/tmp/fdl-compare'
lib = sys.argv[5] if len(sys.argv) > 5 else os.path.join(here, 'lib_clean')
for side, binary in zip('ab', bins):
    env = dict(os.environ, FDL_BIN=os.path.abspath(binary), FDL_NOBLINK='1')
    subprocess.run([sys.executable, os.path.join(here, 'run.py'), script, lib, os.path.join(work, side)], env=env, check=True)
shots = sorted(f for f in os.listdir(os.path.join(work, 'a')) if f.endswith('.png'))
bad = 0
for f in shots:
    pb = os.path.join(work, 'b', f)
    if not os.path.exists(pb):
        print(f'{f}: missing in b'); bad += 1; continue
    a, b = Image.open(os.path.join(work, 'a', f)).convert('RGB'), Image.open(pb).convert('RGB')
    if a.size != b.size:
        print(f'{f}: size {a.size} vs {b.size}'); bad += 1; continue
    d = ImageChops.difference(a, b)
    box = d.getbbox()
    if box is None:
        print(f'{f}: identical'); continue
    n = sum(1 for p in d.getdata() if p != (0, 0, 0))
    print(f'{f}: {n} pixels differ in {box}'); bad += 1
    d.point(lambda v: 255 if v else 0).save(os.path.join(work, f'diff-{f}'))
sys.exit(1 if bad else 0)
