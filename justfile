name := 'cosmic-utils-enroll'
appid := 'org.cosmic_utils.Enroll'

rootdir := ''
prefix := '/usr'

base-dir := absolute_path(clean(rootdir / prefix))

bin-src := 'target' / 'release' / name
bin-dst := base-dir / 'bin' / name

desktop := appid + '.desktop'
desktop-src := 'resources' / desktop
desktop-dst := clean(rootdir / prefix) / 'share' / 'applications' / desktop

appdata := appid + '.metainfo.xml'
appdata-src := 'resources' / appdata
appdata-dst := clean(rootdir / prefix) / 'share' / 'appdata' / appdata

icons-src := 'resources' / 'icons' / 'hicolor'
icons-dst := clean(rootdir / prefix) / 'share' / 'icons' / 'hicolor'

icon-svg-src := icons-src / 'scalable' / 'apps' / 'enroll.svg'
icon-svg-dst := icons-dst / 'scalable' / 'apps' / appid + '.svg'

# Default recipe which runs `just build-release`
default: build-release

# Runs `cargo clean`
clean:
    cargo clean

# Removes vendored dependencies
clean-vendor:
    rm -rf .cargo vendor vendor.tar

# `cargo clean` and removes vendored dependencies
clean-dist: clean clean-vendor

# Compiles with debug profile
build-debug *args:
    cargo build {{ args }}

# Compiles with release profile
build-release *args: (build-debug '--release' args)

# Compiles release profile with vendored dependencies
build-vendored *args: vendor-extract (build-release '--frozen --offline' args)

# Runs a clippy check
check *args:
    cargo clippy --all-features {{ args }} -- -W clippy::pedantic

# Runs a clippy check with JSON message format
check-json: (check '--message-format=json')

# Run the application for testing purposes
run *args:
    env RUST_BACKTRACE=full cargo run --release {{ args }}

# Installs files
install:
    install -Dm0755 {{ bin-src }} {{ bin-dst }}
    install -Dm0644 {{ desktop-src }} {{ desktop-dst }}
    install -Dm0644 {{ appdata-src }} {{ appdata-dst }}
    install -Dm0644 {{ icon-svg-src }} {{ icon-svg-dst }}

# Uninstalls installed files
uninstall:
    rm {{ bin-dst }} {{ desktop-dst }} {{ icon-svg-dst }}

# Vendor dependencies locally
vendor:
    #!/usr/bin/env bash
    mkdir -p .cargo
    cargo vendor --sync Cargo.toml | head -n -1 > .cargo/config.toml
    echo 'directory = "vendor"' >> .cargo/config.toml
    echo >> .cargo/config.toml
    echo '[env]' >> .cargo/config.toml
    if [ -n "${SOURCE_DATE_EPOCH}" ]
    then
        source_date="$(date -d "@${SOURCE_DATE_EPOCH}" "+%Y-%m-%d")"
        echo "VERGEN_GIT_COMMIT_DATE = \"${source_date}\"" >> .cargo/config.toml
    fi
    if [ -n "${SOURCE_GIT_HASH}" ]
    then
        echo "VERGEN_GIT_SHA = \"${SOURCE_GIT_HASH}\"" >> .cargo/config.toml
    fi
    tar pcf vendor.tar .cargo vendor
    rm -rf .cargo vendor

# Extracts vendored dependencies
vendor-extract:
    rm -rf vendor
    tar pxf vendor.tar

# Prepare and publish a release.
#
# Prereq: add your changes under `## [Unreleased]` in CHANGELOG.md.
# Usage:  just release 1.3.0
#
# Bumps the version, regenerates the metainfo <release>, verifies the build
# and that versions are in sync, then commits, tags vX.Y.Z and pushes (which
# triggers the Publish workflow: AppImage + Flathub PR).
release version:
    #!/usr/bin/env bash
    set -euo pipefail

    if [[ -n "$(git status --porcelain)" ]]; then
        echo "error: working tree not clean; commit or stash first" >&2
        exit 1
    fi
    if [[ "$(git rev-parse --abbrev-ref HEAD)" != "main" ]]; then
        echo "error: checkout 'main' first" >&2
        exit 1
    fi
    if git rev-parse -q --verify "refs/tags/v{{ version }}" >/dev/null; then
        echo "error: tag v{{ version }} already exists" >&2
        exit 1
    fi

    # 1. Bump Cargo.toml + finalize the CHANGELOG header for this version.
    python3 .github/workflows/scripts/prepare-release.py "{{ version }}"

    # 2. Verify it compiles; this also syncs Cargo.lock to the new version.
    cargo check

    # 3. Regenerate the metainfo <release> entry from the CHANGELOG section.
    python3 .github/workflows/scripts/update-metainfo.py CHANGELOG.md resources/org.cosmic_utils.enroll.metainfo.xml

    # 4. Hard gate: Cargo.toml version must equal the metainfo top <release>.
    python3 .github/workflows/scripts/check-version-sync.py

    # 5. Optional strict AppStream validation when the tool is available.
    if command -v appstreamcli >/dev/null 2>&1; then
        appstreamcli validate resources/org.cosmic_utils.enroll.metainfo.xml
    else
        echo "note: appstreamcli not installed; skipping strict AppStream validation"
    fi

    # 6. Commit, tag, then confirm the push that triggers publishing.
    git add Cargo.toml Cargo.lock CHANGELOG.md resources/org.cosmic_utils.enroll.metainfo.xml
    git commit -m "release: v{{ version }}" >/dev/null
    git tag "v{{ version }}"

    echo
    echo "Prepared release v{{ version }} (commit + tag created locally)."
    echo "Pushing triggers the Publish workflow (AppImage + Flathub PR)."
    read -r -p "Push to origin and publish? [y/N] " ans
    if [[ "$ans" =~ ^[Yy]$ ]]; then
        git push origin main
        git push origin "v{{ version }}"
        echo "Pushed. https://github.com/cosmic-utils/enroll/actions"
    else
        echo "Aborted. Undo locally: git tag -d v{{ version }} && git reset --hard HEAD~1"
    fi
