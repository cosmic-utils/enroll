# Native packaging

Distribution packaging for Enroll, alongside the existing Flathub build and
AppImage. Each target builds from a `vX.Y.Z` tag, exactly like the Flathub
release flow described in [`../RELEASING.md`](../RELEASING.md).

| Target | Layout | Build model | Distros |
|---|---|---|---|
| AUR | `aur/PKGBUILD` | online `cargo fetch --locked` | Arch |
| Copr | `copr/cosmic-utils-enroll.spec` | **vendored** | Fedora 42+ |
| Launchpad | `debian/*` | **vendored** | Ubuntu 25.04+ |

## Why some targets are vendored

`libcosmic` is a **git dependency** (pinned to a specific rev in `Cargo.lock`),
not published to crates.io. AUR builders have network at build time, so the
PKGBUILD just runs `cargo fetch --locked` and lets Cargo pull libcosmic from
GitHub. Copr and Launchpad build chroots are network-isolated, so those two
targets bundle a **`vendor.tar`** — produced by `just vendor` — which contains
`.cargo/config.toml` + `vendor/` for every transitive dep *including* the
libcosmic git source. The result is a fully reproducible, offline build.

The Rust-version constraint comes from `edition = "2024"` in `Cargo.toml`,
which needs **Rust ≥ 1.85**. That pins the minimum Fedora series to 42 and
the minimum Ubuntu series to 25.04 (plucky). Older series' distro `rustc` is
too old to compile the crate.

## Install layout — single source of truth

All three packagings install via the upstream `just` recipe:

```sh
just rootdir=<destdir> install
```

so the on-disk layout is defined in exactly one place — the `install` recipe
in [`../justfile`](../justfile). Change that recipe and every package follows
automatically. The layout is:

```
/usr/bin/cosmic-utils-enroll
/usr/share/applications/org.cosmic_utils.enroll.desktop
/usr/share/appdata/org.cosmic_utils.enroll.metainfo.xml
/usr/share/icons/hicolor/scalable/apps/org.cosmic_utils.enroll.svg
```

## CI automation

`.github/workflows/publish.yml` adds four jobs that fire on `v*` tag push,
alongside the existing `appimage` and `flathub` jobs. They are **independent**:
a failure in one never suppresses another.

| Job | Produces | Needs |
|---|---|---|
| `vendor-tar` | `vendor.tar` uploaded to the GitHub release | `create-release` |
| `aur` | pushes the updated PKGBUILD + `.SRCINFO` to AUR | `create-release` |
| `copr` | builds an SRPM, submits via `copr-cli` | `vendor-tar` |
| `launchpad` | builds a source package, `dput`s to the PPA | `vendor-tar` |

`vendor-tar` is an enabler — it runs `just vendor` with `SOURCE_DATE_EPOCH` +
`SOURCE_GIT_HASH` set so VERGEN reports the release commit (not the build
host), then uploads the tarball to the GitHub release for `copr` and
`launchpad` to download.

## Required secrets

Set these under *Settings → Secrets and variables → Actions*. The Flathub job
already needs `GH_PAT`; the others are new with this packaging.

| Secret | Used by | How to create |
|---|---|---|
| `AUR_SSH_KEY` | `aur` | A private SSH key whose public half is added to your AUR account under *My Account → SSH Public Keys*. Used to clone+push the AUR package over `aur@aur.archlinux.org`. |
| `COPR_API_TOKEN` | `copr` | Copr → *My Account → API* — paste the whole `~/.config/copr` INI (it holds `login`, `token`, `username`, `copr_url`). Stored as a single multiline secret. |
| `LP_GPG_KEY` | `launchpad` | The **ASCII-armored private key** you've uploaded to Launchpad under *Your profile → OpenPGP keys*. Used to sign the `.changes` file. |
| `LP_GPG_PASSPHRASE` | `launchpad` | The passphrase for `LP_GPG_KEY`. |
| `LP_DPUT_HOST` | `launchpad` | (Optional, override) The `dput` host entry. Defaults to `ppa:cosmic-utils/enroll`. Set if your PPA lives under a different user/name. |

> The Launchpad job also needs the upload to be signed by a key registered to
> the *same* Launchpad account that owns the PPA, or `dput` will reject the
> upload server-side.

## One-time setup per service (out of band)

This repo only contains the packaging files and the CI. Creating the actual
package *targets* on each service is a one-time manual step:

- **AUR** — create the empty package `cosmic-utils/enroll` (or
  `cosmic-utils-enroll` — name must match `pkgname` in the PKGBUILD) via the
  AUR *Submit Package* form. The first `aur` job run will populate it.
- **Copr** — create a project named `enroll` (or whatever you configure),
  enable the Fedora 42+ chroots you want.
- **Launchpad** — create the PPA and register your GPG key.

See [`../RELEASING.md`](../RELEASING.md) for how a release triggers all of
this.
