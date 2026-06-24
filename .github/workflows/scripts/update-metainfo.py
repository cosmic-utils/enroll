import datetime
import re
import sys


def parse_changelog(filepath: str) -> tuple[str, str, list[str]]:
    with open(filepath, "r") as f:
        content = f.read()

    matches = list(
        re.finditer(
            r"^##\s+\[([^\]]+)\](?:\s*-\s*([0-9-]{10}))?", content, re.MULTILINE
        )
    )
    if not matches:
        print("Error: No version headers found in CHANGELOG.md", file=sys.stderr)
        sys.exit(1)

    match = matches[0]
    version = match.group(1)
    date = match.group(2)
    if not date:
        date = datetime.date.today().isoformat()

    start_pos = match.end()
    end_pos = matches[1].start() if len(matches) > 1 else len(content)

    section = content[start_pos:end_pos].strip()

    items: list[str] = []
    for line in section.splitlines():
        line = line.strip()
        if line.startswith("-") or line.startswith("*"):
            item_text = line.lstrip("-* ").strip()
            if item_text:
                items.append(item_text)

    return version, date, items


def update_metainfo(xml_path: str, version: str, date: str, items: list[str]) -> None:
    with open(xml_path, "r") as f:
        xml_content = f.read()

    # Generate the new release block
    release_xml = f'    <release version="{version}" date="{date}">\n'
    release_xml += "      <description>\n"
    release_xml += "        <p>\n"
    release_xml += "          Changes:\n"
    release_xml += "        </p>\n"
    release_xml += "        <ul>\n"
    for item in items:
        escaped_item = (
            item.replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace('"', "&quot;")
        )
        release_xml += f"          <li>{escaped_item}</li>\n"
    release_xml += "        </ul>\n"
    release_xml += "      </description>\n"
    release_xml += "    </release>"

    # Locate version within release tag in XML
    escaped_version = re.escape(version)
    pattern = rf'(\s*<release\s+version=["\']{escaped_version}["\'][^>]*>.*?</release>)'
    new_content, count = re.subn(
        pattern, "\n" + release_xml, xml_content, flags=re.DOTALL
    )

    if count > 0:
        xml_content = new_content
        print(f"Updated existing release {version} in metainfo.")
    else:
        releases_start = xml_content.find("<releases>")
        if releases_start == -1:
            print("Error: <releases> tag not found in metainfo XML", file=sys.stderr)
            sys.exit(1)

        insert_pos = releases_start + len("<releases>")
        xml_content = (
            xml_content[:insert_pos] + "\n" + release_xml + xml_content[insert_pos:]
        )
        print(f"Inserted new release {version} into metainfo.")

    with open(xml_path, "w") as f:
        _ = f.write(xml_content)


if __name__ == "__main__":
    if len(sys.argv) < 3:
        print(
            "Usage: python3 update-metainfo.py <changelog_path> <xml_path>",
            file=sys.stderr,
        )
        sys.exit(1)
    changelog_path = sys.argv[1]
    xml_path = sys.argv[2]
    v, d, items = parse_changelog(changelog_path)
    print(f"Parsed version: {v}, date: {d}, items: {items}")
    update_metainfo(xml_path, v, d, items)
