# Native packaging

Packaging files for distro build services (Fedora Copr and Ubuntu Launchpad), alongside our Flathub and AppImage releases. Builds trigger on `v*` tags following the process in [`RELEASING.md`](RELEASING.md).

| Target | Definitions | Build model | Distros |
|---|---|---|---|
| Copr | `copr/cosmic-utils-enroll.spec` | vendored | Fedora 42+ |
| Launchpad | `debian/*` | vendored | Ubuntu 25.04+ |

## Why vendoring is necessary

`libcosmic` is not on crates.io; Cargo pulls it as a git dependency pinned to a revision in `Cargo.lock`.

That works on local systems and the AUR where builders can hit the network, but Copr and Launchpad build chroots block outbound internet. To build offline, the release workflow runs `just vendor` to produce a `vendor.tar`. It packages `.cargo/config.toml` and a `vendor/` tree containing all transitive dependencies, including `libcosmic`'s git source.

We also require Rust 1.85 or newer because `Cargo.toml` uses `edition = "2024"`. Distro packages for older releases ship with older compilers, so Fedora 42 and Ubuntu 25.04 (Plucky) are our lower bounds.

## Install layout

Both targets install using the `install` recipe from [`../justfile`](../justfile):

```sh
just rootdir=<destdir> install
```

Keeping paths in the root `justfile` means file locations are defined once instead of duplicated across specs:

```
/usr/bin/cosmic-utils-enroll
/usr/share/applications/org.cosmic_utils.enroll.desktop
/usr/share/appdata/org.cosmic_utils.enroll.metainfo.xml
/usr/share/icons/hicolor/scalable/apps/org.cosmic_utils.enroll.svg
```

## CI automation

Tag pushes fire the workflow in `.github/workflows/publish.yml`. It adds three packaging jobs that run alongside the AppImage and Flathub builds:

| Job | Artifact | Depends on |
|---|---|---|
| `vendor-tar` | Uploads `vendor.tar` to GitHub release | `create-release` |
| `copr` | Builds an SRPM, submits via `copr-cli` | `vendor-tar` |
| `launchpad` | Builds a source package, `dput`s to PPA | `vendor-tar` |

The `vendor-tar` job runs first to build the offline source archive. It sets `SOURCE_DATE_EPOCH` and `SOURCE_GIT_HASH` so `vergen` bakes the release commit into the binary rather than the runner's host state. Once uploaded, `copr` and `launchpad` fetch the tarball and run their builds. A failure in one job won't cancel the others.

## Required secrets

Add these in GitHub (*Settings → Secrets and variables → Actions*). The Flathub job already relies on `GH_PAT`; the rest are for the native builds:

| Secret | Used by | Value |
|---|---|---|
| `COPR_API_TOKEN` | `copr` | Raw `~/.config/copr` INI (from Copr → *My Account → API*). Contains `login`, `token`, `username`, and `copr_url`. Paste the whole block as one multiline secret. |
| `LP_GPG_KEY` | `launchpad` | ASCII-armored private key registered with the Launchpad account that owns the PPA (*Your profile → OpenPGP keys*). Signs the `.changes` file. |
| `LP_GPG_PASSPHRASE` | `launchpad` | Passphrase for `LP_GPG_KEY`. |
| `LP_DPUT_HOST` | `launchpad` | Optional. Defaults to `ppa:cosmic-utils/enroll`. Override if using a different PPA name or owner. |

Launchpad will reject the upload if the GPG signature does not match the account that owns the PPA.

## One-time setup

The repository configures the builds, but the target repositories must exist on the remote services before tags are pushed:

- **Copr**: Create the project (default is `enroll`) and enable whichever Fedora 42+ chroots you want to build.
- **Launchpad**: Create the PPA and upload your public GPG key to your profile.

See [`RELEASING.md`](RELEASING.md) for the step-by-step release checklist.
