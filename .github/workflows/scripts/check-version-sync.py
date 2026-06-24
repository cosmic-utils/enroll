#!/usr/bin/env python3
"""Fail if the Cargo.toml version != the top <release> in metainfo.xml.

This is the invariant that makes Flathub builds correct: the commit a tag
points at must ship a metainfo whose latest <release> matches that version.
release-plz violated it (it tagged from Cargo.toml while the metainfo entry
landed on a separate, later-merged PR branch); this guard keeps it honest.

Exits 0 when in sync, 1 otherwise. Run locally (via `just release`) and in CI.
"""

import pathlib
import re
import sys


def repo_root() -> pathlib.Path:
    root = pathlib.Path(__file__).resolve().parent
    while root != root.parent:
        if (root / "Cargo.toml").exists():
            return root
        root = root.parent
    sys.exit("error: could not locate repository root (no Cargo.toml found)")


def main() -> int:
    root = repo_root()

    cargo = (root / "Cargo.toml").read_text()
    m = re.search(r'^version\s*=\s*"([^"]+)"', cargo, re.MULTILINE)
    if not m:
        sys.exit("error: could not find `version` in Cargo.toml")
    cargo_ver = m.group(1)

    xml = (root / "resources" / "org.cosmic_utils.enroll.metainfo.xml").read_text()
    m = re.search(r'<release\s+version="([^"]+)"', xml)
    if not m:
        sys.exit("error: no <release> entry found in metainfo.xml")
    metainfo_ver = m.group(1)

    cl = (root / "CHANGELOG.md").read_text()
    m = re.search(r"^##\s+\[([^\]]+)\]", cl, re.MULTILINE)
    changelog_top = m.group(1) if m else "(none)"

    print(f"Cargo.toml:    {cargo_ver}")
    print(f"metainfo.xml:  {metainfo_ver}")
    print(f"CHANGELOG top: {changelog_top}")

    if cargo_ver != metainfo_ver:
        print(
            f"\nerror: Cargo.toml ({cargo_ver}) and metainfo.xml ({metainfo_ver}) "
            f"are out of sync.\n"
            f"       A tag would ship a build whose metainfo is missing its own "
            f"release entry (breaks Flathub).\n"
            f"       Run `just release {cargo_ver}` to regenerate the metainfo entry.",
            file=sys.stderr,
        )
        return 1

    print("versions in sync")
    return 0


if __name__ == "__main__":
    sys.exit(main())
