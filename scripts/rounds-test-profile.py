"""Builds a ROUNDS profile the way Gale lays one out, from Thunderstore packages, for the rounds.rs test.

    python3 scripts/rounds-test-profile.py <dir>

Every file is a hard link into <dir>/../cache, like Gale's, so the test also shows that cached files are
never written through.
"""
import io, os, shutil, sys, urllib.request, zipfile

PACKAGES = [
    "BepInEx-BepInExPack_ROUNDS-5.4.1901",
    "willis81808-UnboundLib-3.2.14",
    "willis81808-MMHook-1.0.0",
    "olavim-RoundsWithFriends-2.2.2",
    "Pykess-ModdingUtils-0.4.8",
    "olavim-MapsExtended-1.4.2",
    "BossSloth-CardBarPatch-2.1.1",
    "XAngelMoonX-CR-2.7.0",
]

def main(dest):
    cache = os.path.join(os.path.dirname(os.path.abspath(dest)), "cache")
    shutil.rmtree(dest, ignore_errors=True)
    for full in PACKAGES:
        author, name, version = full.split("-", 2)
        pkg = f"{author}-{name}"
        zpath = os.path.join(cache, full + ".zip")
        if not os.path.exists(zpath):
            os.makedirs(cache, exist_ok=True)
            url = f"https://thunderstore.io/package/download/{author}/{name}/{version}/"
            req = urllib.request.Request(url, headers={"User-Agent": "gale-rounds-test"})
            with urllib.request.urlopen(req) as r, open(zpath, "wb") as f:
                f.write(r.read())
        with zipfile.ZipFile(zpath) as z:
            for info in z.infolist():
                path = info.filename.replace("\\", "/")
                if path.endswith("/"):
                    continue
                parts = path.split("/")
                if name.startswith("BepInExPack"):
                    if len(parts) == 1:
                        continue
                    rel = "/".join(parts[1:])
                elif len(parts) == 1 and not path.endswith(".dll"):
                    rel = f"BepInEx/plugins/{pkg}/{path}"
                else:
                    rel = f"BepInEx/plugins/{pkg}/{parts[-1]}"
                cached = os.path.join(cache, full, path)
                os.makedirs(os.path.dirname(cached), exist_ok=True)
                if not os.path.exists(cached):
                    with open(cached, "wb") as f:
                        f.write(z.read(info))
                out = os.path.join(dest, rel)
                os.makedirs(os.path.dirname(out), exist_ok=True)
                if not os.path.exists(out):
                    os.link(cached, out)
    print(f"profile at {dest}, cache at {cache}")

main(sys.argv[1])
