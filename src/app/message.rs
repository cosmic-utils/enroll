// SPDX-License-Identifier: MPL-2.0

use crate::app::AppModel;
use crate::app::error::AppError;
use crate::app::tasks::*;
use crate::app::{
    ContextPage, Finger,
    users::{UserOption, build_nav},
};
use crate::config::{AppTheme, Config};
use crate::fl;
use crate::fprint_dbus::DeviceProxy;
use cosmic::cosmic_config::CosmicConfigEntry;
use cosmic::{Task, command};
use std::sync::Arc;
use tracing::info;
use zbus;

pub const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

/// Messages emitted by the application and its widgets.
#[derive(Debug, Clone)]
pub struct DeviceOption {
    pub path: zbus::zvariant::OwnedObjectPath,
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenRepositoryUrl,
    ToggleContextPage(ContextPage),
    UpdateConfig(Config),
    LaunchUrl(String),
    Delete,
    Register,
    ConnectionReady(zbus::Connection),
    DeviceFound(Option<(zbus::zvariant::OwnedObjectPath, DeviceProxy<'static>)>),
    UpdateDevices(Vec<DeviceOption>),
    OperationError(AppError),
    EnrollStart(Option<u32>),
    EnrollStatus(String, bool),
    EnrollStop,
    DeleteComplete(bool),
    DeleteSingleUnsupported,
    ConfirmDeleteAll,
    CancelDeleteAll,
    ClearDevice,
    CancelClear,
    ClearComplete(Result<(), AppError>),
    CloseApplication,
    EnrolledFingers(Vec<String>),
    FingerSelected(Finger),
    VerifyFinger,
    VerifyStatus(String, bool),
    VerifyStop,
    ThemeChanged(bool),
    ThemeSetting(AppTheme),
    SelectFingerByNumber(u8),
    SelectDevice(usize),
    UsersLoaded(Vec<UserOption>),
}

// Section for handling of Messages
impl AppModel {
    /// Closes the application
    ///
    /// **Return** ***Task***::*done*()
    pub(crate) fn on_close(&mut self) -> Task<cosmic::Action<Message>> {
        Task::done(cosmic::app::Action::Close).map(cosmic::Action::Cosmic)
    }

    /// Resets clear state
    ///
    /// **Returns** ***Task***()
    pub(crate) fn on_cancel_clear(&mut self) -> Task<cosmic::Action<Message>> {
        self.confirm_clear = false;
        Task::none()
    }

    /// After succesfully removal of all prints set status, empties enrolled_fingers
    ///
    /// In case of an *Error* localizes the message and sets status
    ///
    /// **Returns** ***Task***()
    pub(crate) fn on_clear_completion(
        &mut self,
        res: Result<(), AppError>,
    ) -> Task<cosmic::Action<Message>> {
        match res {
            Ok(_) => {
                self.status = fl!("device-cleared");
                self.enrolled_fingers.clear();
            }
            Err(e) => {
                self.status = e.localized_message();
            }
        }
        self.busy = false;
        Task::none()
    }

    /// Opens in a browser clicked hyperlink
    ///
    /// **Returns** ***Task***()
    pub(crate) fn on_clicked_link(&mut self) -> Task<cosmic::Action<Message>> {
        let _ = open::that_detached(REPOSITORY);
        Task::none()
    }

    /// After DBus connection is established searches queries it for fprintd default device
    ///
    /// **Returns** ***task_find_device***(*Connection*)
    pub fn on_connection_ready(&mut self, conn: zbus::Connection) -> Task<cosmic::Action<Message>> {
        self.connection = Some(conn.clone());
        self.status = fl!("status-searching-device");

        let find_conn = conn.clone();
        let devices_conn = conn.clone();
        Task::batch(vec![
            task_find_device(find_conn),
            get_devices_task(devices_conn),
        ])
    }

    /// Toggles the context page
    ///
    /// **Returns** ***Task***()
    pub(crate) fn on_context_page_toggle(
        &mut self,
        context_page: ContextPage,
    ) -> Task<cosmic::Action<Message>> {
        if self.context_page == context_page {
            // Close the context drawer if the toggled context page is the same.
            self.core.window.show_context = !self.core.window.show_context;
        } else {
            // Open the context drawer to display the requested context page.
            self.context_page = context_page;
            self.core.window.show_context = true;
        }
        Task::none()
    }

    /// Localizes the error and stores it on status resetting everything
    ///
    /// **Returns** ***Task***()
    pub(crate) fn on_error(&mut self, err: AppError) -> Task<cosmic::Action<Message>> {
        if err == AppError::NoEnrolledPrints {
            self.enrolled_fingers.clear();
            self.status = fl!("success");
        } else {
            self.status = err.localized_message();
        }
        self.busy = false;
        self.enrolling_finger = None;
        Task::none()
    }

    /// Stores the results of list_fingers_task
    ///
    /// **Returns** ***Task***()
    pub(crate) fn on_fingers_listed(
        &mut self,
        fingers: Vec<String>,
    ) -> Task<cosmic::Action<Message>> {
        self.enrolled_fingers = fingers;
        Task::none()
    }

    /// If device is not busy compares localized string to fingers and set matching to be
    /// the selected one
    ///
    /// **Returns** ***Task***()
    pub(crate) fn on_finger_selected(&mut self, finger: Finger) -> Task<cosmic::Action<Message>> {
        if self.busy {
            return Task::none();
        }
        self.confirm_clear = false;
        self.selected_finger = finger;
        Task::none()
    }

    /// Stores fingerprint scanner devices received
    ///
    /// Return ***Task***::**none**()
    pub(crate) fn on_devices_found(
        &mut self,
        devices: Vec<DeviceOption>,
    ) -> Task<cosmic::Action<Message>> {
        self.devices = devices;
        Task::none()
    }

    /// Requests users enrolled prints
    ///
    /// **Returns** either ***Task***::**none**() or ***list_fingers_task***()
    pub(crate) fn on_device_found(
        &mut self,
        device_info: Option<(zbus::zvariant::OwnedObjectPath, DeviceProxy<'static>)>,
    ) -> Task<cosmic::Action<Message>> {
        if let Some((path, proxy)) = device_info {
            self.device_path = Some(Arc::new(path));
            self.device_proxy = Some(proxy);
            self.status = fl!("status-device-found");
            self.busy = false;

            if self.selected_user.is_some() {
                self.list_fingers_task()
            } else {
                Task::none()
            }
        } else {
            self.device_path = None;
            self.device_proxy = None;
            self.status = fl!("status-no-device-found");
            self.busy = true;
            Task::none()
        }
    }

    /// Called to request verification of the selected print
    ///
    /// **Returns** ***Task***()
    pub(crate) fn on_verify_finger(&mut self) -> Task<cosmic::Action<Message>> {
        if self.busy {
            return Task::none();
        }
        if self
            .enrolled_fingers
            .iter()
            .any(|ef| ef == self.selected_finger.as_finger_id())
        {
            self.busy = true;
            self.verifying_finger = true;
            self.status = fl!("status-starting-verification");
        }
        Task::none()
    }

    /// Handles verification status updates
    ///
    /// **Returns** ***Task***()
    pub(crate) fn on_verify_status(
        &mut self,
        status: String,
        done: bool,
    ) -> Task<cosmic::Action<Message>> {
        // Here you could map verify-* strings to localized messages
        // Currently we'll fallback to showing the string directly or success message if done
        let status_msg = match status.as_str() {
            "verify-match" => fl!("verify-match"),
            "verify-no-match" => fl!("verify-no-match"),
            "verify-retry-scan" => fl!("verify-retry-scan"),
            "verify-swipe-too-short" => fl!("verify-swipe-too-short"),
            "verify-finger-not-centered" => fl!("verify-finger-not-centered"),
            "verify-remove-and-retry" => fl!("verify-remove-and-retry"),
            "verify-too-fast" => fl!("verify-too-fast"),
            "verify-disconnected" => fl!("verify-disconnected"),
            "verify-unknown-error" => fl!("verify-unknown-error"),
            "verify-cancelled" => fl!("verify-cancelled"),
            _ => status.clone(),
        };
        self.status = status_msg;

        if done {
            self.busy = false;
            self.verifying_finger = false;
        }
        Task::none()
    }

    /// Stops any ongoing verification
    pub(crate) fn on_verify_stop(&mut self) -> Task<cosmic::Action<Message>> {
        if let (Some(path), Some(conn)) = (self.device_path.clone(), self.connection.clone()) {
            let path = (*path).clone();
            task_verify_stop(path, conn)
        } else {
            Task::none()
        }
    }

    /// Starts the enroll process, set status and enroll options
    ///
    /// **Returns** ***Task***()
    pub(crate) fn on_enroll_start(&mut self, total: Option<u32>) -> Task<cosmic::Action<Message>> {
        self.enroll_total_stages = total;
        self.enroll_progress = 0;
        self.status = fl!("enroll-starting");
        Task::none()
    }

    /// Takes responses from Fprintd API and converts them to localized strings
    ///
    /// Set status and ends process when it is done
    ///
    /// **Returns** ***Task***()
    pub(crate) fn on_enroll_status(
        &mut self,
        status: String,
        done: bool,
    ) -> Task<cosmic::Action<Message>> {
        let status_msg = match status.as_str() {
            "enroll-stage-passed" => {
                self.enroll_progress += 1;
                fl!("enroll-stage-passed")
            }
            "enroll-retry-scan" => fl!("enroll-retry-scan"),
            "enroll-swipe-too-short" => fl!("enroll-swipe-too-short"),
            "enroll-finger-not-centered" => fl!("enroll-finger-not-centered"),
            "enroll-remove-and-retry" => fl!("enroll-remove-and-retry"),
            "enroll-unknown-error" => fl!("enroll-unknown-error"),
            "enroll-completed" => fl!("enroll-completed"),
            "enroll-failed" => fl!("enroll-failed"),
            "enroll-disconnected" => fl!("enroll-disconnected"),
            "enroll-data-full" => fl!("enroll-data-full"),
            "enroll-too-fast" => fl!("enroll-too-fast"),
            "enroll-duplicate" => fl!("enroll-duplicate"),
            "enroll-cancelled" => fl!("enroll-cancelled"),
            _ => status.clone(),
        };
        self.status = status_msg;

        if done {
            self.busy = false;
            self.enrolling_finger = None;

            if status == "enroll-completed" {
                let cycle = self.on_cycle_finger(1);
                return Task::batch(vec![cycle, self.list_fingers_task()]);
            }
        }
        Task::none()
    }

    /// Sends stop signal to end an ongoing enroll process
    ///
    /// **Returns** either ***Task***() or ***task_enroll_stop***()
    pub(crate) fn on_enroll_stop(&self) -> Task<cosmic::Action<Message>> {
        if self.enrolling_finger.is_none() {
            return Task::none();
        }
        if let (Some(path), Some(conn)) = (self.device_path.clone(), self.connection.clone()) {
            let path = (*path).clone();
            return task_enroll_stop(path, conn);
        }
        Task::none()
    }

    /// Clears all prints for all users
    ///
    /// **Returns** either ***Task***() or ***task_clear_device***()
    pub(crate) fn on_clear_device(&mut self) -> Task<cosmic::Action<Message>> {
        if self.busy {
            return Task::none();
        }

        if !self.confirm_clear {
            self.confirm_clear = true;
            return Task::none();
        }

        if let (Some(path), Some(conn)) = (&self.device_path, &self.connection) {
            self.status = fl!("clearing-device");
            self.busy = true;
            self.confirm_clear = false;
            let usernames: Vec<String> = self.users.iter().map(|u| (*u.username).clone()).collect();
            return task_clear_device(path.as_ref().to_owned(), usernames, conn.clone());
        }
        Task::none()
    }

    /// Deletes the selected finger's print for the current user.
    ///
    /// **Returns** either ***Task***() or ***task_delete_print***()
    pub(crate) fn on_delete(&mut self) -> Task<cosmic::Action<Message>> {
        if self.busy {
            return Task::none();
        }

        if let (Some(path), Some(conn), Some(user)) = (
            self.device_path.clone(),
            self.connection.clone(),
            self.selected_user.clone(),
        ) {
            self.status = fl!("deleting");
            self.busy = true;
            let path = (*path).clone();
            let username = (*user.username).clone();

            let finger_name = self.selected_finger.as_finger_id().to_string();
            return task_delete_print(path, username, finger_name, conn);
        }
        Task::none()
    }

    /// Set state when deletion of prints was succesful and removes from enrolled_fingers
    ///
    /// **Returns** ***Task***()
    pub(crate) fn on_delete_complete(&mut self, clear: bool) -> Task<cosmic::Action<Message>> {
        self.status = fl!("deleted");
        self.busy = false;

        if clear {
            self.enrolled_fingers.clear();
        } else {
            self.enrolled_fingers
                .retain(|f| f.as_str() != self.selected_finger.as_finger_id());
        }

        Task::none()
    }

    /// Single-finger delete is unsupported by the running fingerprint service;
    /// offer to delete all of the user's prints instead via a dialog.
    ///
    /// **Returns** ***Task***()
    pub(crate) fn on_delete_single_unsupported(&mut self) -> Task<cosmic::Action<Message>> {
        self.busy = false;
        self.confirm_delete_all = true;
        self.status = fl!("delete-all-fallback");
        Task::none()
    }

    /// User chose to delete all of the selected user's prints after the
    /// single-finger delete fallback dialog.
    ///
    /// **Returns** ***task_delete_prints***() or ***Task***::**none**()
    pub(crate) fn on_confirm_delete_all(&mut self) -> Task<cosmic::Action<Message>> {
        self.confirm_delete_all = false;
        if let (Some(path), Some(conn), Some(user)) = (
            self.device_path.clone(),
            self.connection.clone(),
            self.selected_user.clone(),
        ) {
            self.status = fl!("deleting");
            self.busy = true;
            let path = (*path).clone();
            let username = (*user.username).clone();
            return task_delete_prints(path, username, conn);
        }
        Task::none()
    }

    /// User cancelled the single-finger delete fallback dialog.
    ///
    /// **Returns** ***Task***()
    pub(crate) fn on_cancel_delete_all(&mut self) -> Task<cosmic::Action<Message>> {
        self.confirm_delete_all = false;
        Task::none()
    }

    /// Opens given Uniform Resourse Locator
    ///
    /// **Returns** ***Task***()
    pub(crate) fn on_open_link(&mut self, url: String) -> Task<cosmic::Action<Message>> {
        match open::that_detached(&url) {
            Ok(()) => Task::none(),
            Err(err) => {
                eprintln!("failed to open {url:?}: {err}");
                Task::none()
            }
        }
    }

    /// Sets state as busy and sets which finger is being registered for subscription
    ///
    /// **Returns** ***Task***()
    pub(crate) fn on_register(&mut self) -> Task<cosmic::Action<Message>> {
        if !self.busy && self.device_path.is_some() && self.enrolling_finger.is_none() {
            self.busy = true;
            self.enrolling_finger = Some(Arc::new(self.selected_finger.as_finger_id().to_string()));
            self.status = fl!("status-starting-enrollment");
        }
        Task::none()
    }

    /// Sets the config state as the given on and writes it to disk
    ///
    /// **Returns** ***Task***()
    pub(crate) fn on_update_config(&mut self, config: Config) -> Task<cosmic::Action<Message>> {
        self.config = config.clone();

        if let Some(handler) = &self.config_handler
            && let Err(err) = config.write_entry(handler)
        {
            tracing::error!("failed to write config: {}", err);
        }

        Task::none()
    }

    /// On Flatpak non COSMIC DE sets Theme if set to System
    ///
    /// **Returns** ***cosmic::command::set_theme***() or ***None***()
    pub(crate) fn on_portal_color_scheme_changed(
        &mut self,
        is_dark: bool,
    ) -> Task<cosmic::Action<Message>> {
        use crate::config::AppTheme;

        // Only apply if user wants to follow system theme
        if self.config.app_theme != AppTheme::System {
            return Task::none();
        }

        info!(is_dark, "Portal color scheme changed, updating theme");
        let theme = if is_dark {
            cosmic::Theme::dark()
        } else {
            cosmic::Theme::light()
        };
        command::set_theme(theme)
    }

    pub fn on_theme_setting(&mut self, theme: AppTheme) -> Task<cosmic::Action<Message>> {
        self.config.app_theme = theme;

        if let Some(handler) = &self.config_handler
            && let Err(err) = self.config.write_entry(handler)
        {
            tracing::error!("failed to write config: {}", err);
        }

        cosmic::command::set_theme(theme.theme())
    }

    /// Selects a finger by numeric key (1-0).
    ///
    /// **Returns** ***Task***()
    pub(crate) fn on_select_finger_by_number(&mut self, key: u8) -> Task<cosmic::Action<Message>> {
        if let Some(finger) = Finger::from_key(key)
            && !self.busy
        {
            self.confirm_clear = false;
            self.selected_finger = finger;
        }
        Task::none()
    }

    pub(crate) fn on_select_device(&mut self, index: usize) -> Task<cosmic::Action<Message>> {
        if self.busy {
            return Task::none();
        }

        if let Some(device) = self.devices.get(index)
            && let Some(conn) = &self.connection
        {
            self.status = fl!("status-searching-device");
            self.busy = true;
            return task_select_device(conn.clone(), device.path.clone());
        }
        Task::none()
    }

    /// Cycles through selectable fingers.
    ///
    /// **Returns** ***Task***()
    pub(crate) fn on_cycle_finger(&mut self, direction: i8) -> Task<cosmic::Action<Message>> {
        if self.busy {
            return Task::none();
        }
        let fingers = Finger::all();
        if let Some(pos) = fingers.iter().position(|f| *f == self.selected_finger) {
            let len = fingers.len() as i8;
            let next = ((pos as i8 + direction) % len + len) % len;
            self.confirm_clear = false;
            self.selected_finger = fingers[next as usize];
        }
        Task::none()
    }

    /// Handles asynchronously loaded user list, builds nav bar and selects current user.
    ///
    /// **Returns** ***update_title_task***() and ***list_fingers_task***()
    pub(crate) fn on_users_loaded(
        &mut self,
        users: Vec<UserOption>,
    ) -> Task<cosmic::Action<Message>> {
        let (nav, selected_user) = build_nav(&users);
        self.nav = nav;
        self.users = users;
        self.selected_user = selected_user;

        Task::batch(vec![self.update_title_task(), self.list_fingers_task()])
    }
}
