# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.2.0](https://github.com/cosmic-utils/enroll/releases/tag/1.2.0) - 2026-06-24

### Added

- prompt to delete all when single-finger delete is unsupported
- support deleting fingerprints with open-fprintd
- added Badges to README
- support multiple scanner devices
- user avatar support
- support for getting all scanner devices
- Added Cancel button to Verify operation
- add delete all user fingerprints option
- Better present enrollment progress

### Fixed

- show clear status message when fprintd lacks a method
- incorrect URL
- add closing release tag
- remove & char
- release section had & character
- added cancel button to view, simplified sub
- typo in metainfo description

### Other

- v1.2.0
- Rename CONTRIBUTING.md to .github/CONTRIBUTING.md
- Rename RELEASE.md to RELEASING.md
- Rename README.md to .github/README.md
- add AppImage as part of a release
- *(cicd)* update release info with changes from CHANGELOG
- *(cargo)* update
- update reflecting changes
- fix CI/CD to use a fork
- new CI/CD
- Release process
- CHANGELOG formatting change
- bump version numbers, update release information
- updated dependencies
- Merge pull request #138 from cosmic-utils/135-add-provide-info
- add CosmicApplication id to provides tag
- *(metadata)* add pictures and change colors
- update change information
- *(settings)* added the device name
- *(settings)* add localization, show number of devices
- preparing for release
- flatpak-cargo-generator.py
- update dependencies
- check reference
- move final task into tasks
- replace unwrap with matching Options
- *(analyzer)* skip a macro
- optimize user change
- prepare 1.0.9 for release
- add usage help page and license info into about
- *(metadata)* update finnish text for Flathub
- update release info
- update cargo-source.json before release
- making verify tasks signature match with enroll counterpart
- update tag to next release
- version bump, updated dependencies
- reoder metainfo description paragraphs
- *(cs)* Update Czech translation
- Preparing for release
- Add files via upload
- Updated projects own CHANGELOG as well
- Italian metadata and latest changes
- Pressing number no longer changes if device busy
- Fixed test of number key to finger and comment
- Unit test
- Changed from_key to match order from screen
- Moved key_subcription to subscription.rs
- Initial implementation of keyboard navigation
- Version up to 1.0.6 and cargo update
- Removed unused import
- Final release change tweaks
- Reset window limits to work with almost all mobiles
- Finished touches on Settings so all (en) strings render correct
- Moved settings function implementation to own file.
- Fluent new danger string
- Release information updated
- Ran clippy --fix
- Fixed clippy warnings about ? operator
- Read config in init to be able to set theme
- Using theme method from AppTheme to set Theme.
- Refine usage instructions in README.md
- Update Arch Linux desktop environment in README
- Revise README for clarity and additional details
- Whitespace surrounding icon
- Added swedish to metainfo
- Added all translation string to fi & sv
- set display length to window width of 400px
- Corrected metadata by using markdown not HTML
- Merge branch '1.0.4' into app_metadata
- New main photo and update release info
- Added new release to metadata
- Changelog and formatting
- Added preliminary Theming support
- Added one more screenshot
- Bump version number
- Fixes bug where Verify doesn't release claimed device
- Replaced COSMIC with System in .desktop file
- Preparing for Flathub
- Bump version number
- Changed GitHub action to a full release by default
- Fixed Verify button being enabled for fingers with on enrolled prints
- Bump version number
- Changed default nav_bar state to untoggled
- Flathub manifest
- Flatpak related changes
- Removed Cargo.lock
- Changed translation string used when user has no prints recorded
- Removed old logic for handling completed deletion.
- Changed order of start and status in verify
- Return from verify in case of failure and remove on_success
- Color enrolled fingers svg using themes success color
- Final comment adjustments
- Factored creating subscriptions for verify and enroll into their own functions
- Renamed verify_finger_process to better reflect its content
- Simplistic Success replaced with EnrollStatus
- New subscription for verifying fingerprints
- Boolean for verifying_finger subscription included into AppModel
- New translation string status-starting-verification & verify-retry-scan
- Removed unused files. Changed Icon in .desktop
- Polished README and CONTRIBUTING
- Doc comments for tasks
- Clippy was shouting again
- Removed &self from a couple of tasks and moved them to tasks.rs
- Revert "Renamed mod.rs model.rs"
- Renamed mod.rs model.rs
- Removed task functions
- Moved message.rs task helper functions to tasks.rs
- Fixed how CHANGELOG looks rendered.
- First full featured release
- Fix broken links in
- Bump version number
- Renamed CHANGES to CHANGELOG.md.
- Refactor tasks into separate functions
- Finished doc comments for Message handlers
- Merge branch '0.5.5' of https://github.com/cosmic-utils/enroll into 0.5.5
- More Doc comments and factoring out tasks into functions
- Doc comments
- Added unit test for MenuAction::Settings
- Improved doc comments
- Added more doc comments
- Put columnt with svg and text into container to combat no tooltip issue
- Added comments to the net.reactived.Fprintd API calling functions
- appl.rs to application.rs. Purely aesthetical change.
- Made icons be symbolic (renderer can choose color)
- Modified CHANGES to reflect latest developments
- Added localization string. Changed default UI. Removed unused things and suppressed warnings where there still is a TODO.
- Ran cargo update, iced was upgraded to 0.14 by libcosmic.
- Added comment & TODO
- Fixed verify by looping over signal status
- Translation strings for verify statuses
- Bump version number
- New CHANGES
- Moved clearing all prints to Settings
- Second pass at the alternative UI
- Reverted back to text button for now
- Initialize Verify button and added tooltips with translations
- Experimenting with custom_image_button on other UI
- Added Verify fprintd API, Messages & button
- Added few translation strings
- Added latest changes
- Fixed clippy warnings
- Renamed app.rs to appl.rs (short of application) for clippy
- First pass at creating the alternative UI
- Refactored Tasks into functions
- Added UI segment to Settings
- Internationalized one more string
- Moved Message handler functions part of AppModel impl to message.rs
- Added view.rs
- Moved part of AppModel impl, specifically view functions, to view.rs
- Bump version up
- Made checkbox just hide/display the title for now
- Fixed clippy warnings
- Added first configurable value & checkbox for it in Settings
- Version bump and changes
- Miscellaneous maintenance stuff
- Moved user related things to its own file
- Added Settings menu
- Version bump
- Revamped the UI a bit
- Reverted back to only accepting users with icon files. Added TODO's for implementing user icons.
- Use symbolic icon for now
- Update CHANGES
- Reverted the UI closer to what it was
- Now return multiple tasks from on_nav_select to also run list_fingers_task on user change
- User fetching synchronously & icon fixed
- Merge branch 'v0.5.0' of https://github.com/cosmic-utils/enroll into v0.5.0
- Modified column in the view
- Fixed enrolling_finger was the wrong string issue
- Continuing the refactor to have users in navbar
- DeleteComplete had old Page related things
- Now that Page is Finger and default finger is Finger things got simpler
- Updated user & added icon
- Clippy warnings
- Storing the path to user icon
- Renamed Page enum to Finger
- Formatted code and bumped version to v0.5.0
- Renamed icon.svg to enroll.svg so changed references to it
- Renamed icon.svg to enroll.svg and made changes reflecting that
- Modified the workflow to match
- Renamed the CI file as well.
- Renamed to prepare for Flathub submission
- Missed a couple at Fprint's in longer lines
- Translation files from cosmic_ext_fprint to cosmic_utils_enroll
- cosmic-utils to cosmic_utils
- Rolled back a change accidentally commited
- Added request about opening PR's
- Refactored README to be user focused and added CONTRIBUTING.md for more developer focused stuff
- Renamed the files to match
- Renamed Fprint to Enroll
- Renamed from fi.joonastuomi.Fprint to org.cosmic-utils.Enroll
- Made the icon much simpler for scalability
- New icon and version bump
- Merge pull request #81 from jotuel/security/validate-username-12948134448153143460
- Merge pull request #84 from jotuel/perf/enrollment-optimization-17165333057639684485
- Optimize startup by loading config asynchronously
- Updated translation
- Use delete_enrolled_fingers2 to remove all users prints
- Merge branch 'v0.3.11' into clear-device-feature-8400983118941603589
- Merge pull request #76 from jotuel/feature-delete-all-prints-2767584120024299952
- Upgrade CI image to official one
- Upgrade freedesktop Platform v24 to 25
- Final update to what changed before release
- Added finnish translation
- Added translation change
- Bumped version to 0.3.10
- Improved phrasing
- Better window limits
- Merge pull request #70 from jotuel/test-user-option-display-6314129216703630380
- Add unit tests for UserOption::fmt implementation
- Bump version from 0.3.8 to 0.3.9
- Update CHANGES for version 0.3.9
- Moved dependency & asset licenses to THIRD_PARTY_LICENSES
- Update to LICENSE and SPDX-LicenseIdentifiers
- Fixed clippy
- Merge branch 'v0.3.9' into refactor/update-method-8186401395515008297
- Refactor AppModel::update to use helper methods
- One more reference
- Found a couple more old ids
- Bump version from 0.3.7 to 0.3.8
- Update CHANGES for version 0.3.8
- Replace old screen recording link with new one
- Revise README for clarity and additional details
- Rename app title to 'Fprint Enroll'
- Rename Fprint Extension to Fprint
- Rename application from CosmicFprint to Fprint
- Rename repository to cosmic-ext-fprint and remove COSMIC branding
- Bump version from 0.3.6 to 0.3.7
- Update CHANGES file for version 0.3.7
- Merge branch 'v0.3.7' into perf-optimization-subscription-allocations-16398452532923199718
- Optimize subscription generation by reducing redundant allocations
- Simplified removing deleted print from listed prints
- Add user selection dropdown
- Add user selection dropdown
- Add user selection dropdown
- Add user selection dropdown
- Changed icon.svg background alpha to 0
- Bumped the version number up
- Merge branch 'main' into refactor-split-app-rs-11637322954988485641
- Refactor app.rs into multiple modules
- Improved error handling by always releasing the device
- Improve fprintd enrollment messages and error handling
- Improve fprintd enrollment messages and error handling
- Improve fprintd enrollment messages and error handling
- Tweaked the icon just a bit
- Continued removing dead code and fixed icon
- Added tracing, uncommented the code and removed unused code.
- Refactor repetitive navigation item initialization
- Merge branch 'main' into fix-libcosmic-trait-bound-9239294558807106097
- Merge branch 'main' into perf-dbus-connection-13044524613160494545
- Reuse persistent DBus connection for device operations
- add unit tests for map_finger_name in app.rs
- Add video link for fingerprint registration demo
- Enhance README with usage and installation information
- Bumped minor version number upp.
- Removed libfprint-rs bindings to libfprint
- Refactor fprintd interaction to use zbus D-Bus client
- Added the necessary things to use new file in the project.
- Created a file for fingerprint reading related functions.
- Changed appid to reflect my new (old) domain.
- Bumped edition to 2024 to be able to build it and added libfprint-rs as
- Track changes to project.
- Added libfprint-rs as a dependancy for libfprint bindings.
- Added a fingerprint svg. Also now stdout is returned as a String and
- Deleted emoji from english locale.
- Create fprint.svg
- Create LICENSE
- The code is a wee bit on the ugly side still but deletion and addition of fingerprints is functional. No listing though. And still no user feedback through GUI.
- Registering fingerprints now works. If no fingerprint is not selected from nav menu app panics. Feedback is not yet provided back to the user through GUI.
- I am starting to understand the iced model and made some inroads towards implementing rudimentary logic. Because executing cmd'sseems simple in Rust this is actually the hardest part.
- Added page for each possible fingerprint slot on Goodix reader.
- This just launches an application. So now the work begins.
- Project scaffolding up
- Create main.rs
- Initial commit

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
