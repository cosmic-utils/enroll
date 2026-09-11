### Enroll is fingerprint manager software
###
### You don't have to follow these exactly just keep the users in mind.
###

## Most prominent parts of user interface.
## 

# Header message.
app-title = Enroll Fingerprint
# View menu item for opening About section.
about = About
# View menu item for opening Settings section.
settings = Settings
# View menu item for opening Help section.
help = Help
# Menu in the header that contains About, Settings & Help sections.
view = View
# Fprint used to the name of the application but it was renamed as Enroll. Use Enroll, or translate it as something else.
fprint = Enroll
# Welcome message
welcome = Register and/or delete fingerprints
# In About section there is a link to the git commit of the current build and when it was build.
git-description = Git commit { $hash } on { $date }
# Text displayed on a button that registers a fingerprint.
register = Register
# Text displayed on a button that verifies a fingerprint.
verify = Verify
# Text displayed on a button that deletes a fingerprint.
delete = Delete
# Section header in Settings that has button to empty the whole scanner device.
danger = Danger
# Button to Cancel deletion operation on a confirmation dialog.
cancel = Cancel
# This is status cover message for a really rare state. It conveys that everything is OK and you can keep using the application.
success = Success. Now go register a print.
# Status messages that a fingerprint was succesfully deleted.
deleted = Deleted fingerprint.
# Fingerprint deletion is underway status message.
deleting = Deleting fingerprint…
# Button that attempts to clear the entire scanner device.
delete-all = Delete all
# Someone using open-fprintd, or other implementation, not supporting deleting a single fingerprint, this dialog and options is given.
delete-all-fallback = This fingerprint service does not support deleting a single fingerprint. Delete all of this user's fingerprints instead?
# This element contains the Delete All button.
clear-device = Clear Device
# Dialog message that confirms you really wanted deletion of all prints.
confirm-clear = Are you sure?
# Status shown while all users every print is being deleted.
clearing-device = Clearing all fingerprints from device for all known users…
# Status when succesfully cleared the whole scanner device.
device-cleared = Device cleared for all known users.
# Dialog message that confirms you really wanted deletion of all prints.
clear-device-confirm = Are you sure you want to clear fingerprints for ALL known users?

## Licensing, Settings and Help related content. Some links are still English only.
## 

# Section that contains link to licensing information of it and all its dependencies.
about-licenses = License information
# Tells user that fprintd is needed (in Flathub it is not possible to mandate this) and has a link to fprintd webpage.
help-fprintd = If you have any issues please first check you have fprintd correctly installed. You can find more information from their website:
# Additional information about how to possibly get more, especially Validity, scanners working using open-fprintd.
help-validity = For some hardware, like Validity scanners for example, you need to currently use open-fprintd instead.
# Link to English written Arch Wiki section about Pluggable Authentication Modules to set it up.
help-pam = Also to be able to utilize fingerprint effectively PAM must be configured to use fprintd module for authentication. You can find examples here:
# Section where there still is a toggle for using the old interface.
settings-ui = User Interface
# The toggle of old interface.
alternative-ui = Alternative UI
# Settings section for removal of all fingerprints.
settings-clear-device = Remove all fingerprints
# Theme related Setting
settings-theme = Theme
# Tells you how many supported devices fprintd found. Then lists them by name and lets you select which to use.
settings-device =
    { $nbr ->
        [1] You have { $nbr } supported device
       *[other] You have { $nbr } supported devices
    }
# Follow System Theme.
theme-system = System
# Use Light Theme.
theme-light = Light
# Use Dark Theme.
theme-dark = Dark

## Messages that appear when you hover a button or icon for additional context.
## 

# Message displayed when you hover over Register button.
register-tooltip = Registers a new fingerprint
# Message displayed when you hover over Delete button.
delete-tooltip = Deletes this fingerprint
# Message displayed when you hover over Delete All button.
clear-tooltip = Only works for still existing users
# Message displayed when you hover over Verify button.
verify-tooltip = Verifies the fingerprint against the enrolled fingerprints

## Messages from fprintd Verify API adjusted to be less technical and more helpful.
## 

# Status when you started to Verify.
verify-finger = Place { $finger } on reader
# Displayed when Verify returns no.
verify-no-match = Fingerprint did not match
# Displayed when Verify returns yes.
verify-match = Fingerprint matched
# Scanner could not Verify.
verify-retry-scan = Could not read fingerprint. Try again.
# Scanner could not Verify. Needs a longer swipe.
verify-swipe-too-short = Swipe was too short. Try again.
# Scanner could not Verify. Finger needs to be in the center of the scanner.
verify-finger-not-centered = Finger was not centered. Try again.
# Scanner could not Verify. Remove finger from the scanner before attempting again.
verify-remove-and-retry = Remove finger and try again.
# Scanner could not Verify. Needs a longer touch.
verify-too-fast = Too fast touch. Try again.
# Scanner could not Verify because device connection was lost. User must close the whole application before attempting again.
verify-disconnected = Device disconnected. Close window.
# Scanner could not Verify for unknown reason.
verify-unknown-error = There was an unknown error.
# Verify was cancelled.
verify-cancelled = Verification cancelled.

## Finger names. page is left as a historical artifact.
## 

# Name of right thumb
page-right-thumb = Right Thumb
# Name of right index
page-right-index-finger = Right Index Finger
# Name of right middle one
page-right-middle-finger = Right Middle Finger
# Name of right ring one
page-right-ring-finger = Right Ring Finger
# Name of right little one
page-right-little-finger = Right Little Finger
# Name of left thumb
page-left-thumb = Left Thumb
# Name of left index
page-left-index-finger = Left Index Finger
# Name of left middle one
page-left-middle-finger = Left Middle Finger
# Name of left ring one
page-left-ring-finger = Left Ring Finger
# Name of left little one
page-left-little-finger = Left Little Finger
# Name for deleting them all
page-delete-all-users-prints = Delete All User's Prints

## These cover gap states left by fprintd in the application to keep users informed.
## 

# Message displayed whilst application connects to DBus.
status-connecting = Connecting to system bus…
# Message displayed whilst fprintd searches for compatible devices.
status-searching-device = Searching for fingerprint reader…
# Message displayed after a succesful connection to both DBus and scanner device is established. Ready to use.
status-device-found = Choose which fingerprint to register.
# Message displayed when no fprintd compatible devices were found.
status-no-device-found = No fingerprint reader found.
# Message displayed whilst fprintd waits for scanner hardware to signal it is ready to start registration.
status-starting-enrollment = Starting enrollment…
# Message displayed whilst fprintd waits for scanner hardware to signal it is ready to match a print.
status-starting-verification = Starting verification…

## Fprintd Enroll API messages converted into less technical, more human, ones.
## 

# When the device is ready and waiting for the first touch.
enroll-starting = Place your finger on the reader to start.
# Previous touch succesfully registered but more are needed to complete a record.
enroll-stage-passed = Scan successful. Keep going.
# Scanner device was unable to register the touch and you need to redo it.
enroll-retry-scan = Could not read fingerprint. Please try again.
# Swiping scanner hardware returns this message when you need to do longer swipe.
enroll-swipe-too-short = Swipe was too short. Please swipe the full length of the sensor.
# When you need to redo it but place finger in the center of the scanner.
enroll-finger-not-centered = Finger not centered. Please place your finger in the middle of the sensor.
# You need to lift your finger and place it back down again.
enroll-remove-and-retry = Please remove your finger and try again.
# Scanner hardware told there was an error but not why.
enroll-unknown-error = Unknown error occurred.
# Succesfully recorded a fingerprint record.
enroll-completed = Fingerprint enrolled.
# Registering record to the device failed.
enroll-failed = Enrollment failed.
# Scanner device contact lost.
enroll-disconnected = Device disconnected.
# Scanner hardware did not have sufficient space for any new records and some must be freed up.
enroll-data-full = Fingerprint storage is full. Please delete some fingerprints.
# Swiping scanner tells you must redo the swipe slower.
enroll-too-fast = Swipe was too fast. Please swipe slower.
# The fingerprint is already registered either as a different finger or for a different user.
enroll-duplicate = This finger is already enrolled.
# User cancelled the registering process.
enroll-cancelled = Enrollment cancelled.

## Fprintd API Error messages programmer to human conversions.
## 

# Whatever you tried to do fprintd could not get permission for it from the system.
error-permission-denied = Permission denied.
# Scanner hardware is already in use for something else.
error-already-in-use = Device is already in use by another application.
# An unknown error occured in scanner hardware.
error-internal = An internal error occurred.
# Trying to match a finger for a user for which it does not exist.
error-no-enrolled-prints = No fingerprints enrolled for this finger.
# Fprintd could not reserve the device because something has not unclaimed it.
error-claim-device = Could not claim the device.
# An operation was attempted without any fprintd compatible hardware. If this ever happens there is a flaw in the application.
error-device-not-found = Fingerprint device not found.
# User took too long.
error-timeout = Operation timed out.
# Deleting prints failed.
error-prints-not-deleted = Could not delete fingerprints.
# DBus connection failed and message from DBus is printed as-is for debugging purposes.
error-connect-dbus = Failed to connect to DBus: { $err }
# The version of fprintd used does not support the whole fprintd API.
error-unsupported-operation = This operation is not supported by the fingerprint service. The installed fprintd implementation may be incompatible.
