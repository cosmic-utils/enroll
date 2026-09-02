#!/usr/bin/env python3
"""Update Debian changelog and Copr RPM spec for a new release.

Usage: update-packaging.py <version> [changelog_path]

Extracts items from the latest entry in CHANGELOG.md (or defaults to "Release <version>.")
and prepends entries to:
  - packaging/debian/changelog
  - packaging/copr/cosmic-utils-enroll.spec
"""

import datetime
import email.utils
import pathlib
import re
import sys

MAINTAINER_NAME = "Joonas Tuomi"
MAINTAINER_EMAIL = "git@joonastuomi.fi"
PACKAGE_NAME = "cosmic-utils-enroll"


def repo_root() -> pathlib.Path:
    root = pathlib.Path(__file__).resolve().parent
    while root != root.parent:
        if (root / "Cargo.toml").exists():
            return root
        root = root.parent
    sys.exit("error: could not locate repository root (no Cargo.toml found)")


def parse_latest_changelog(filepath: pathlib.Path) -> tuple[str, list[str]]:
    """Return (version, list of bullet items) from the top release in CHANGELOG.md."""
    if not filepath.exists():
        return "", []
    content = filepath.read_text(encoding="utf-8")
    matches = list(
        re.finditer(
            r"^##\s+\[([^\]]+)\](?:\s*-\s*([0-9-]{10}))?", content, re.MULTILINE
        )
    )
    if not matches:
        return "", []

    version = matches[0].group(1)
    start_pos = matches[0].end()
    end_pos = matches[1].start() if len(matches) > 1 else len(content)
    section = content[start_pos:end_pos].strip()

    items: list[str] = []
    for line in section.splitlines():
        line = line.strip()
        if line.startswith("-") or line.startswith("*"):
            item_text = line.lstrip("-* ").strip()
            if item_text:
                items.append(item_text)

    return version, items


def update_debian_changelog(
    deb_path: pathlib.Path, version: str, items: list[str], now: datetime.datetime
) -> None:
    rfc2822_date = email.utils.format_datetime(now)
    entry_version = f"{version}-1"

    bullets = "\n".join(f"  * {item}" for item in items) if items else f"  * Release {version}."
    new_entry = (
        f"{PACKAGE_NAME} ({entry_version}) unstable; urgency=medium\n\n"
        f"{bullets}\n\n"
        f" -- {MAINTAINER_NAME} <{MAINTAINER_EMAIL}>  {rfc2822_date}\n"
    )

    if deb_path.exists():
        current_content = deb_path.read_text(encoding="utf-8")
        # If this exact version is already at the top of the changelog, replace it
        top_match = re.match(
            rf"^{re.escape(PACKAGE_NAME)}\s+\({re.escape(entry_version)}\)[^\n]*\n.*?\n -- [^\n]+\n+",
            current_content,
            re.DOTALL,
        )
        if top_match:
            updated = new_entry + "\n" + current_content[top_match.end() :]
        else:
            updated = new_entry + "\n" + current_content.lstrip()
    else:
        updated = new_entry

    deb_path.write_text(updated, encoding="utf-8")
    print(f"Updated {deb_path.relative_to(repo_root())} -> {entry_version}")


def update_copr_spec(
    spec_path: pathlib.Path, version: str, items: list[str], now: datetime.datetime
) -> None:
    if not spec_path.exists():
        print(f"warning: {spec_path} does not exist, skipping", file=sys.stderr)
        return

    content = spec_path.read_text(encoding="utf-8")

    # Update Version: <version>
    content, count_v = re.subn(
        r"^Version:\s+.*$",
        f"Version:        {version}",
        content,
        count=1,
        flags=re.MULTILINE,
    )
    if count_v == 0:
        sys.exit(f"error: could not find 'Version:' in {spec_path}")

    # Reset Release: 1%{?dist}
    content, _ = re.subn(
        r"^Release:\s+.*$",
        "Release:        1%{?dist}",
        content,
        count=1,
        flags=re.MULTILINE,
    )

    # Format RPM changelog date: Day Mon DD YYYY (e.g. Tue Jul 15 2026)
    rpm_date = now.strftime("%a %b %d %Y")
    bullets = "\n".join(f"- {item}" for item in items) if items else f"- Release {version}."
    changelog_header = f"* {rpm_date} {MAINTAINER_NAME} <{MAINTAINER_EMAIL}> - {version}-1"
    new_changelog_entry = f"{changelog_header}\n{bullets}\n"

    # Insert after %changelog line
    changelog_marker = "%changelog\n"
    idx = content.find(changelog_marker)
    if idx == -1:
        sys.exit(f"error: could not find '%changelog' in {spec_path}")

    rest = content[idx + len(changelog_marker) :]
    # If the top changelog entry is already for this version, replace it
    pattern = rf"^\* [^\n]+ - {re.escape(version)}-1\n.*?(?=(\n\* |\Z))"
    top_entry_match = re.match(pattern, rest, re.DOTALL)
    if top_entry_match:
        updated_rest = new_changelog_entry + "\n" + rest[top_entry_match.end() :].lstrip("\n")
    else:
        updated_rest = new_changelog_entry + "\n" + rest.lstrip("\n")

    updated = content[: idx + len(changelog_marker)] + updated_rest
    spec_path.write_text(updated, encoding="utf-8")
    print(f"Updated {spec_path.relative_to(repo_root())} -> {version}-1")


def main() -> int:
    if len(sys.argv) < 2 or len(sys.argv) > 3:
        print("usage: update-packaging.py <version> [changelog_path]", file=sys.stderr)
        return 2

    version = sys.argv[1]
    root = repo_root()
    changelog_path = (
        pathlib.Path(sys.argv[2]) if len(sys.argv) == 3 else (root / "CHANGELOG.md")
    )

    cl_ver, items = parse_latest_changelog(changelog_path)
    if cl_ver and cl_ver != version:
        print(
            f"note: CHANGELOG top version is [{cl_ver}], but updating packaging to target [{version}]",
            file=sys.stderr,
        )

    now = datetime.datetime.now(datetime.timezone.utc)
    deb_path = root / "packaging" / "debian" / "changelog"
    spec_path = root / "packaging" / "copr" / "cosmic-utils-enroll.spec"

    update_debian_changelog(deb_path, version, items, now)
    update_copr_spec(spec_path, version, items, now)
    return 0


if __name__ == "__main__":
    sys.exit(main())
