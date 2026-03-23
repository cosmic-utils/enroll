// SPDX-License-Identifier: MPL-2.0

use crate::app::AppModel;
use crate::app::error::AppError;
use crate::app::tasks::*;
use crate::app::{ContextPage, Finger};
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
pub enum Message {
    OpenRepositoryUrl,
    ToggleContextPage(ContextPage),
    UpdateConfig(Config),
    LaunchUrl(String),
    Delete,
    Register,
    ConnectionReady(zbus::Connection),
    DeviceFound(Option<(zbus::zvariant::OwnedObjectPath, DeviceProxy<'static>)>),
    OperationError(AppError),
    EnrollStart(Option<u32>),
    EnrollStatus(String, bool),
    EnrollStop,
    DeleteComplete(bool),
    ClearDevice,
    CancelClear,
    ClearComplete(Result<(), AppError>),
    EnrolledFingers(Vec<String>),
    FingerSelected(String),
    VerifyFinger,
    VerifyStatus(String, bool),
    ThemeChanged(bool),
    ThemeSetting(AppTheme),
}

// Section for handling of Messages
impl AppModel {
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
    pub(crate) fn on_connection_ready(
        &mut self,
        conn: zbus::Connection,
    ) -> Task<cosmic::Action<Message>> {
        self.connection = Some(conn.clone());
        self.status = fl!("status-searching-device");

        let conn_clone = conn.clone();
        task_find_device(conn_clone)
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
        self.status = err.localized_message();
        if self.status == fl!("error-no-enrolled-prints") {
            self.enrolled_fingers.clear();
            self.status = fl!("success");
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
    pub(crate) fn on_finger_selected(&mut self, finger: String) -> Task<cosmic::Action<Message>> {
        if self.busy {
            return Task::none();
        }
        self.confirm_clear = false;
        for fingers in Finger::all() {
            if fingers.localized_name() == finger {
                self.selected_finger = *fingers;
                break;
            }
        }
        Task::none()
    }

    /// Requests users enrolled prints
    ///
    /// **Returns** either ***Task***() or ***list_fingers_task***()
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
        self.busy = true;
        self.verifying_finger = true;
        self.status = fl!("status-starting-verification");
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
            _ => status.clone(),
        };
        self.status = status_msg;

        if done {
            self.busy = false;
            self.verifying_finger = false;
        }
        Task::none()
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
                return self.list_fingers_task();
            }
        }
        Task::none()
    }

    /// Sends stop signal to end an ongoing enroll process
    ///
    /// **Returns** either ***Task***() or ***task_enroll_stop***()
    pub(crate) fn on_enroll_stop(&self) -> Task<cosmic::Action<Message>> {
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
        if !self.confirm_clear {
            self.confirm_clear = true;
            return Task::none();
        }

        if let (Some(path), Some(conn)) = (self.device_path.clone(), self.connection.clone()) {
            self.status = fl!("clearing-device");
            self.busy = true;
            self.confirm_clear = false;
            let path = (*path).clone();
            let usernames: Vec<String> = self.users.iter().map(|u| (*u.username).clone()).collect();
            return task_clear_device(path, usernames, conn);
        }
        Task::none()
    }

    /// Deletes chosen print or users all prints depending on choices from the user
    ///
    /// **Returns** either ***Task***(), ***task_delete_print***() or ***task_delete_prints***()
    pub(crate) fn on_delete(&mut self) -> Task<cosmic::Action<Message>> {
        if let (Some(path), Some(conn), Some(user)) = (
            self.device_path.clone(),
            self.connection.clone(),
            self.selected_user.clone(),
        ) {
            self.status = fl!("deleting");
            self.busy = true;
            let path = (*path).clone();
            let username = (*user.username).clone();

            if let Some(finger_name) = self.selected_finger.as_finger_id() {
                let finger_name = finger_name.to_string();
                return task_delete_print(path, username, finger_name, conn);
            } else {
                return task_delete_prints(path, username, conn);
            }
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
                .retain(|f| f != self.selected_finger.as_finger_id().unwrap());
        }

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
        self.busy = true;
        if let Some(finger_id) = self.selected_finger.as_finger_id() {
            self.enrolling_finger = Some(Arc::new(finger_id.to_string()));
        }
        self.status = fl!("status-starting-enrollment");
        Task::none()
    }

    /// Sets the config state as the given on and writes it to disk
    ///
    /// **Returns** ***Task***()
    pub(crate) fn on_update_config(&mut self, config: Config) -> Task<cosmic::Action<Message>> {
        self.config = config.clone();

        if let Some(handler) = &self.config_handler {
            if let Err(err) = config.write_entry(handler) {
                tracing::error!("failed to write config: {}", err);
            }
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
        let mut tasks = vec![self.on_update_config(Config {
            app_theme: theme,
            ..self.config.clone()
        })];

        let task = cosmic::command::set_theme(theme.theme());

        tasks.push(task);
        Task::batch(tasks)
    }
}
