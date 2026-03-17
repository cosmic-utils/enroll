### 1.0.4:
  AppID changed to org.cosmic-utils.Enroll 
### 1.0.3:
  Fixes a bug in Verify option
### 1.0.2:
  • Fixes Verify being active for unenrolled fingers
  • Add cargo-source.json for Flathub
### 1.0.1:
  • Changed navigation bar default toggle state
  • Pushed Cargo.lock for Flathub
### 1.0.0:
	• First fully featured release
	• Adds Settings section for installing dependencies
	• Removing all users prints added to new main UI
### 0.5.5:
	Symbolic SVGs & commiting config changes to disk
### 0.5.4:
	Default UI is the new one
### 0.5.3:
	New alternative UI & config option for it
### 0.5.2:
	• Implemented Settings menu
	• Refactored user options
### 0.5.1:
	Added placeholder user icon
### 0.5.0:
	Refactored users to nav & fingerprint picker
### 0.4.1:
	• Switched to snake case in ID
	• Renamed icon.svg to enroll.svg
### 0.4.0:
	• Moved project to COSMIC utils
	• Renamed from Fprint to Enroll
	• Redesigned icon svg
### 0.3.12:
	• Performance improvements
	• Updated icon
### 0.3.11:
	• Implement a confirmation dialog for "Clear Device" operation
	• Fix missing bulk deletion logic for all fingerprints of a user
### 0.3.10:
	• Fallback to finding user with libc instead of enviroment variable
	• Swedish translation thanks to @bittin
	• Finnish translation
### 0.3.9:
	• Improved licensing by adding identifiers to all files and adding full copy of it
	• Maintainability improvements by structuring the code into smaller functions
### 0.3.8:
  	• Changed repository name to cosmic-ext-utils for legal reasons
  	• Replaced COSMIC Fprint ids with just fi.joonastuomi.Fprint or Fprint
### 0.3.7:
  	• Added localization string
  	• More idiomatic Rust
    • Improved memory reusage
### 0.3.6:
  	Add user selection dropdown
### 0.3.5:
  	• Disable delete button if fingerprint is not enrolled
  	• Performance optimization: avoid redundant string clone in signal handler
### 0.3.4:
  	Added Flatpak
### 0.3.3:
  	Refactored init to use loops
### 0.3.2:
  	Improved error responses
### 0.3.1:
  	Better localization for responses from the daemon
### 0.3.0:
  	Switch to using DBus and net.reactived.Fprint
### 0.2.0:
  	Update dependencies and improve fingerprint enrollment flow by using libfprint-rs 0.3.1
### 0.1.0:
  	First release of cosmic-fprint
