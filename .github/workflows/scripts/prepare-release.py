#!/usr/bin/env python3
"""Bump Cargo.toml to <version> and finalize the CHANGELOG header for a release.

Called by `just release <version>`. It only does the text edits:
  - Cargo.toml:  version = "<version>"
  - CHANGELOG:   rename a leading `## [Unreleased]` header (or an existing
                 header for this exact version) into `## [<version>](url) - <today>`

It deliberately does NOT touch metainfo.xml (that is update-metainfo.py's job,
run immediately after) and does NOT tag (the recipe tags last). Keeping these
concerns separate makes the release ordering invariant provable: by the time a
tag exists, Cargo.toml, CHANGELOG and metainfo all already agree on the version.
"""

import datetime
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
    if len(sys.argv) != 2:
        print("usage: prepare-release.py <version>", file=sys.stderr)
        return 2
    version = sys.argv[1]
    if not re.fullmatch(r"\d+\.\d+\.\d+", version):
        print(
            f"error: version must be X.Y.Z (e.g. 1.3.0), got {version!r}",
            file=sys.stderr,
        )
        return 2

    today = datetime.date.today().isoformat()
    url = f"https://github.com/cosmic-utils/enroll/releases/tag/v{version}"
    root = repo_root()

    cargo = root / "Cargo.toml"
    ct, n = re.subn(
        r'^version\s*=\s*"[^"]+"',
        f'version = "{version}"',
        cargo.read_text(),
        count=1,
        flags=re.MULTILINE,
    )
    if n != 1:
        print(
            'error: could not find a `version = "..."` line in Cargo.toml',
            file=sys.stderr,
        )
        return 1
    cargo.write_text(ct)

    cl = root / "CHANGELOG.md"
    t = cl.read_text()
    t2, n = re.subn(
        r"^##\s+\[Unreleased\][^\n]*$",
        f"## [{version}]({url}) - {today}",
        t,
        count=1,
        flags=re.MULTILINE,
    )
    if n == 0:
        # No Unreleased section: accept a header already written for this version.
        pat = rf"^##\s+\[{re.escape(version)}\](?:\([^)]*\))?[^\n]*$"
        t2, n = re.subn(
            pat, f"## [{version}]({url}) - {today}", t, count=1, flags=re.MULTILINE
        )
    if n == 0:
        top = re.search(r"^##\s+\[([^\]]+)\]", t, re.MULTILINE)
        found = top.group(1) if top else "(none)"
        print(
            f"error: CHANGELOG top entry is [{found}], expected [Unreleased] or [{version}].\n"
            f"       Add your release notes under a '## [Unreleased]' section first.",
            file=sys.stderr,
        )
        return 1
    cl.write_text(t2)

    print(f"Bumped Cargo.toml -> {version}; finalized CHANGELOG header ({today}).")
    return 0


if __name__ == "__main__":
    sys.exit(main())
