"""Copies what the ROUNDS layer ships into src-tauri/resources/rounds: from a ROUNDS Porting Toolkit checkout the
hand-made patches, the Odin stand-in and the list of old library releases; from a DuctTape checkout AutoFix and the
Runtime (build them first, Release).

    python scripts/rounds-sync.py [<toolkit> [<ducttape>]]      default: ../toolkit-repo and ../DuctTape next to this one

Prints what changed; run it and rebuild when either changes.
"""
import filecmp, os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
TK = os.path.abspath(sys.argv[1] if len(sys.argv) > 1 else os.path.join(HERE, "..", "..", "toolkit-repo"))
DT = os.path.abspath(sys.argv[2] if len(sys.argv) > 2 else os.path.join(HERE, "..", "..", "DuctTape"))
RES = os.path.join(HERE, "..", "src-tauri", "resources", "rounds")
BIN = lambda p, f: os.path.join(DT, "src", p, "bin", "Release", "net472", f)

changed = []


def put(src, dest):
    if not os.path.isfile(src): sys.exit(f"missing {src} (build DuctTape first?)")
    if os.path.isfile(dest) and filecmp.cmp(src, dest, shallow=False): return
    os.makedirs(os.path.dirname(dest), exist_ok=True)
    shutil.copyfile(src, dest)
    changed.append(os.path.relpath(dest, RES))


put(os.path.join(TK, "patches", "patches.tsv"), os.path.join(RES, "patches.tsv"))
used = {l.split("\t")[3].strip() for l in open(os.path.join(TK, "patches", "patches.tsv"), encoding="utf-8") if l.strip()}
for name in used:
    put(os.path.join(TK, "patches", name), os.path.join(RES, "patches", name))
for name in os.listdir(os.path.join(RES, "patches")):
    if name not in used:
        os.remove(os.path.join(RES, "patches", name)); changed.append(f"removed patches/{name}")
put(os.path.join(TK, "data", "old-libraries.tsv"), os.path.join(RES, "old-libraries.tsv"))
put(BIN("AutoFix", "rounds-port.AutoFix.dll"), os.path.join(RES, "files", "rounds-port.AutoFix.dll"))
put(BIN("Runtime", "rounds-port.Runtime.dll"), os.path.join(RES, "files", "rounds-port.Runtime.dll"))
for name in os.listdir(os.path.join(TK, "odin")):
    put(os.path.join(TK, "odin", name), os.path.join(RES, "files", "odin", name))

print("\n".join(changed) if changed else "already up to date")
