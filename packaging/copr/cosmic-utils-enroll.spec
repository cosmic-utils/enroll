# SPDX-License-Identifier: MPL-2.0
#
# cosmic-utils-enroll — RPM spec for Fedora Copr.
#
# Vendored, fully offline build: Copr chroots fetch no network at build time,
# and libcosmic is a git dependency (not on crates.io), so the SRPM bundles
# `vendor.tar` (produced by the `vendor-tar` CI job from the justfile
# `just vendor` recipe). CI produces Source0 via `git archive` of the tag.
#
# Target chroots: Fedora 42+ (rustc >= 1.85, needed for Rust edition 2024).
# Older Fedora has too-old rustc.

%global appname      cosmic-utils-enroll
%global appid        org.cosmic_utils.enroll
%global forgeurl     https://github.com/cosmic-utils/enroll

# Let rpmbuild compute the libxkbcommon soname dep automatically.
%global __provides_exclude_from ^%{_libdir}/%{name}/.*$

Name:           cosmic-utils-enroll
Version:        1.2.1
Release:        1%{?dist}
Summary:        GUI for fprintd fingerprint enrolling (COSMIC)

License:        MPL-2.0
URL:            %{forgeurl}
# Source0  — git archive of the tag, produced by CI (packaging/publish.yml).
# Source1  — vendor.tar from `just vendor` (CI `vendor-tar` job).
Source0:        %{name}-%{version}.tar.gz
Source1:        vendor.tar

BuildRequires:  cargo >= 1.85
BuildRequires:  fontconfig-devel
BuildRequires:  freetype-devel
BuildRequires:  gcc
BuildRequires:  just
BuildRequires:  libxkbcommon-devel
BuildRequires:  make
BuildRequires:  pkgconf-pkg-config
BuildRequires:  rustc >= 1.85
BuildRequires:  wayland-devel

# The app is useless without the D-Bus daemon it drives.
Requires:       fprintd
# Runtime .so deps are picked up by rpm's auto-dependency generator
# (libxkbcommon.so.0, libgcc_s.so.1, glibc).

%description
Enroll is a COSMIC GUI for fprintd fingerprint enrolling. It can register,
verify and delete fingerprint records, and manage prints for another user on
multi-user systems.

%prep
%setup -q -a1
# %setup -a1 unpacks vendor.tar on top of the source tree, leaving
#   .cargo/config.toml and vendor/ next to Cargo.toml — exactly what
#   `just vendor-extract` would have produced.

%build
# VERGEN env is baked into .cargo/config.toml by `just vendor`, so the
# binary reports the release commit, not the build machine's git state.
cargo build --release --frozen --offline

%install
# Reuse the upstream install recipe verbatim — the on-disk layout stays
# defined in exactly one place (the justfile).
just rootdir=%{buildroot} install

%check
# No upstream test suite; the %build compile is the gate.
# No-op keeps the stage so a future harness is wired in automatically.

%files
%license LICENSE
%{_bindir}/%{appname}
%{_datadir}/applications/%{appid}.desktop
%{_datadir}/appdata/%{appid}.metainfo.xml
%{_datadir}/icons/hicolor/scalable/apps/%{appid}.svg

%changelog
* Tue Jul 15 2026 Joonas Tuomi <joonas@cosmic-utils.org> - 1.2.1-1
- Initial Fedora spec for Copr.
