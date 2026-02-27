// SPDX-License-Identifier: MPL-2.0

use std::{collections::HashMap, sync::Arc};

use cosmic::widget::{menu, nav_bar};

use crate::{
    app::{
        finger::{ContextPage, Finger},
        message::{Message, UserOption},
    },
    config::Config,
    fprint_dbus::DeviceProxy,
};

pub mod error;
pub mod finger;
pub mod fprint;
pub mod message;

/// The application model stores app-specific state used to describe its interface and
/// drive its logic.
pub struct AppModel {
    /// Application state which is managed by the COSMIC runtime.
    core: cosmic::Core,
    /// Display a context drawer with the designated page if defined.
    context_page: ContextPage,
    /// Contains items assigned to the nav bar panel.
    nav: nav_bar::Model,
    /// Key bindings for the application's menu bar.
    key_binds: HashMap<menu::KeyBind, MenuAction>,
    // Configuration data that persists between application runs.
    config: Config,
    // Status text for the UI
    status: String,
    // Currently selected device path
    device_path: Option<Arc<zbus::zvariant::OwnedObjectPath>>,
    // Reused device proxy
    device_proxy: Option<DeviceProxy<'static>>,
    // Shared DBus connection
    connection: Option<zbus::Connection>,
    // Whether an operation is in progress
    busy: bool,
    // Finger currently being enrolled (None if not enrolling)
    enrolling_finger: Option<Arc<String>>,
    // Enrollment progress
    enroll_progress: u32,
    // If device supports num_enroll_stages a Some(u32) else None
    enroll_total_stages: Option<u32>,
    // List of users (username, realname)
    users: Vec<UserOption>,
    // Selected user
    selected_user: Option<UserOption>,
    // Selected finger
    selected_finger: Finger,
    // List of enrolled fingers
    enrolled_fingers: Vec<String>,
    // Confirmation state for clearing the device
    confirm_clear: bool,
}

mod app;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuAction {
    About,
}

impl menu::action::MenuAction for MenuAction {
    type Message = Message;

    fn message(&self) -> Self::Message {
        match self {
            MenuAction::About => Message::ToggleContextPage(ContextPage::About),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::app::error::AppError;

    use super::*;
    use cosmic::widget::menu::action::MenuAction as _;

    #[test]
    fn test_app_error_localization() {
        // Test localized message for permission denied
        assert_eq!(
            AppError::PermissionDenied.localized_message(),
            "Permission denied."
        );
        // Test localized message for already in use
        assert_eq!(
            AppError::AlreadyInUse.localized_message(),
            "Device is already in use by another application."
        );
        // Test localized message for device not found
        assert_eq!(
            AppError::DeviceNotFound.localized_message(),
            "Fingerprint device not found."
        );
        // Test localized message for timeout
        assert_eq!(
            AppError::Timeout.localized_message(),
            "Operation timed out."
        );
        // Test localized message for DBus connection error
        assert_eq!(
            AppError::ConnectDbus("Connection error".to_string()).localized_message(),
            "Failed to connect to DBus: \u{2068}Connection error\u{2069}"
        );
    }

    #[test]
    fn test_app_error_unknown_context() {
        let err = AppError::Unknown("Some error".to_string());
        let err_with_context = err.with_context("Context");

        assert_eq!(err_with_context.localized_message(), "Context: Some error");
    }

    #[test]
    fn test_app_error_known_context() {
        // Context should be ignored for known errors
        let err = AppError::PermissionDenied;
        let err_with_context = err.with_context("Context");

        assert_eq!(err_with_context.localized_message(), "Permission denied.");
    }

    #[test]
    fn test_menu_action_message() {
        let action = MenuAction::About;
        assert!(matches!(
            action.message(),
            Message::ToggleContextPage(ContextPage::About)
        ));
    }
}
