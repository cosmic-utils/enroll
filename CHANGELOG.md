# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.1] - 2026-05-25

### Fixed
- Fixes incorrect metadata

## [1.1.0] - 2026-05-19

### Added
- Support for multiple scanner devices
- Display user avatar on multi-user systems

## [1.0.9] - 2026-05-06

### Added
- New Help page into View menu

### Changed
- Improved fi translations

## [1.0.8] - 2026-04-16

### Added
- Cancel option to an ongoing Verify process

### Changed
- Updated sv & cz translation by @bittin, @lorduskordus

### Removed
- Removed custom tab handling as updated libcosmic works now

## [1.0.7] - 2026-04-08

### Added
- Ukrainian translation by @Dymkom
- Added initial Tab support

## [1.0.6] - 2026-04-02

### Added
- Italian translations by @albannobattistella
- Buttons 1-0, v, r & ctrl + d added

### Changed
- libcosmic update

## [1.0.5] - 2026-03-24

### Added
- Config loaded at startup
- Theme applied from config

### Changed
- Updated Czech translation by @lorduskordus

### Fixed
- Fixed inconsistency in app limits and metadata by @1peter10

## [1.0.4] - 2026-03-22

### Added
- Add ashpd for Flatpak theming support
- Czech translation by @lorduskordus

### Changed
- Updated app metadata

## [1.0.3] - 2026-03-17

### Added
- First Flathub release

### Fixed
- Fixes a bug in Verify option

## [1.0.2] - 2026-03-16

### Added
- Add cargo-source.json for Flathub

### Fixed
- Fixes Verify being active for unenrolled fingers

## [1.0.1] - 2026-03-15

### Added
- Pushed Cargo.lock for Flathub

### Changed
- Changed navigation bar default toggle state

## [1.0.0] - 2026-03-12

### Added
- First fully featured release
- Adds Settings section for installing dependencies
- Removing all users prints added to new main UI

## [0.5.5] - 2026-03-11

### Added
- Symbolic SVGs & commiting config changes to disk

## [0.5.4] - 2026-03-09

### Changed
- Default UI is the new one

## [0.5.3] - 2026-03-08

### Added
- New alternative UI & config option for it

## [0.5.2] - 2026-03-05

### Added
- Implemented Settings menu

### Changed
- Refactored user options

## [0.5.1] - 2026-02-28

### Added
- Added placeholder user icon

## [0.5.0] - 2026-02-27

### Changed
- Refactored users to nav & fingerprint picker

## [0.4.1] - 2026-02-25

### Changed
- Switched to snake case in ID
- Renamed icon.svg to enroll.svg

## [0.4.0] - 2026-02-23

### Changed
- Moved project to COSMIC utils
- Renamed from Fprint to Enroll
- Redesigned icon svg

## [0.3.12] - 2026-02-22

### Changed
- Performance improvements
- Updated icon

## [0.3.11] - 2026-02-20

### Added
- Implement a confirmation dialog for "Clear Device" operation

### Fixed
- Fix missing bulk deletion logic for all fingerprints of a user

## [0.3.10] - 2026-02-19

### Added
- Swedish translation thanks to @bittin
- Finnish translation

### Changed
- Fallback to finding user with libc instead of enviroment variable

## [0.3.9] - 2026-02-18

### Changed
- Improved licensing by adding identifiers to all files and adding full copy of it
- Maintainability improvements by structuring the code into smaller functions

## [0.3.8] - 2026-02-16

### Changed
- Changed repository name to cosmic-ext-utils for legal reasons
- Replaced COSMIC Fprint ids with just fi.joonastuomi.Fprint or Fprint

## [0.3.7] - 2026-02-15

### Added
- Added localization string

### Changed
- More idiomatic Rust
- Improved memory reusage

## [0.3.6] - 2026-02-14

### Added
- Add user selection dropdown

## [0.3.5] - 2026-02-14

### Changed
- Disable delete button if fingerprint is not enrolled
- Performance optimization: avoid redundant string clone in signal handler

## [0.3.4] - 2026-02-13

### Added
- Added Flatpak

## [0.3.3] - 2026-02-13

### Changed
- Refactored init to use loops

## [0.3.2] - 2026-02-13

### Changed
- Improved error responses

## [0.3.1] - 2026-02-13

### Changed
- Better localization for responses from the daemon

## [0.3.0] - 2026-01-04

### Changed
- Switch to using DBus and net.reactived.Fprint

## [0.2.0] - 2025-11-17

### Changed
- Update dependencies and improve fingerprint enrollment flow by using libfprint-rs 0.3.1

## [0.1.0] - 2025-06-22

### Added
- First release of cosmic-fprint
