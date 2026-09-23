// SPDX-License-Identifier: MPL-2.0

use std::{collections::HashMap, sync::Arc};

use cosmic::{
    cosmic_config,
    widget::{about::About, menu, nav_bar, segmented_button},
};

use crate::{
    app::{
        finger::Finger,
        message::{DeviceOption, Message},
        users::UserOption,
    },
    config::{AppTheme, Config},
    fl,
    fprint_dbus::DeviceProxy,
};

pub mod error;
pub mod finger;
pub mod fprint;
pub mod message;
pub mod settings;
pub mod subscription;
pub mod tasks;
pub mod users;
pub mod view;

/// Application model stores app-specific state
///
/// Describes interface and
/// drives its logic
pub struct AppModel {
    /// Application state which is managed by the COSMIC runtime.
    core: cosmic::Core,
    /// Display a context drawer with the designated page if defined.
    context_page: ContextPage,
    /// About context drawer
    about: About,
    /// Contains items assigned to the nav bar panel.
    nav: nav_bar::Model,
    /// Theme selection
    theme: segmented_button::SingleSelectModel,
    /// Key bindings for the application's menu bar.
    key_binds: HashMap<menu::KeyBind, MenuAction>,
    // Configuration data that persists between application runs.
    config: Config,
    // Config handler for writing & reading it
    config_handler: Option<cosmic_config::Config>,
    // Status text for the UI
    status: String,
    // Currently selected device path
    device_path: Option<Arc<zbus::zvariant::OwnedObjectPath>>,
    // All devices
    devices: Vec<DeviceOption>,
    // Reused device proxy
    device_proxy: Option<DeviceProxy<'static>>,
    // Shared DBus connection
    connection: Option<zbus::Connection>,
    // Whether an operation is in progress
    busy: bool,
    // Finger currently being enrolled (None if not enrolling)
    enrolling_finger: Option<Arc<String>>,
    // Whether verifying a finger
    verifying_finger: bool,
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
    // Confirm deleting a fingerprint
    confirm_delete: bool,
    // Confirmation state for clearing the device
    confirm_clear: bool,
    // Confirmation state for falling back to deleting all of a user's prints
    // when single-finger delete is unsupported (e.g. open-fprintd).
    confirm_delete_all: bool,
}

mod application;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuAction {
    About,
    Settings,
    Help,
    SelectFinger(u8),
    Delete,
    Register,
    Verify,
    Cancel,
    Quit,
}

impl menu::action::MenuAction for MenuAction {
    type Message = Message;

    fn message(&self) -> Self::Message {
        match self {
            MenuAction::About => Message::ToggleContextPage(ContextPage::About),
            MenuAction::Settings => Message::ToggleContextPage(ContextPage::Settings),
            MenuAction::Help => Message::ToggleContextPage(ContextPage::Help),
            MenuAction::SelectFinger(x) => Message::SelectFingerByNumber(*x),
            MenuAction::Delete => Message::Delete,
            MenuAction::Register => Message::Register,
            MenuAction::Verify => Message::VerifyFinger,
            MenuAction::Cancel => Message::Stop,
            MenuAction::Quit => Message::CloseApplication,
        }
    }
}

/// Returns the default key bindings for the application's menu bar.
pub fn default_key_binds() -> HashMap<menu::KeyBind, MenuAction> {
    use cosmic::iced::keyboard::Key;
    use cosmic::iced::keyboard::key::Named;
    use cosmic::widget::menu::key_bind::{KeyBind, Modifier};

    let mut key_binds = HashMap::new();
    key_binds.insert(
        KeyBind {
            modifiers: vec![],
            key: Key::Named(Named::F1),
        },
        MenuAction::Help,
    );
    key_binds.insert(
        KeyBind {
            modifiers: vec![Modifier::Ctrl],
            key: Key::Character(",".into()),
        },
        MenuAction::Settings,
    );
    key_binds.insert(
        KeyBind {
            modifiers: vec![Modifier::Ctrl],
            key: Key::Character("i".into()),
        },
        MenuAction::About,
    );
    key_binds.insert(
        KeyBind {
            modifiers: vec![Modifier::Ctrl],
            key: Key::Character("q".into()),
        },
        MenuAction::Quit,
    );
    key_binds.insert(
        KeyBind {
            modifiers: vec![Modifier::Ctrl],
            key: Key::Character("c".into()),
        },
        MenuAction::Cancel,
    );
    for x in 0..=9 {
        key_binds.insert(
            KeyBind {
                modifiers: vec![],
                key: Key::Character(x.to_string().into()),
            },
            MenuAction::SelectFinger(x),
        );
    }
    key_binds.insert(
        KeyBind {
            modifiers: vec![Modifier::Ctrl],
            key: Key::Character("d".into()),
        },
        MenuAction::Delete,
    );
    key_binds.insert(
        KeyBind {
            modifiers: vec![],
            key: Key::Character("r".into()),
        },
        MenuAction::Register,
    );
    key_binds.insert(
        KeyBind {
            modifiers: vec![],
            key: Key::Character("v".into()),
        },
        MenuAction::Verify,
    );
    key_binds
}

/// The context page to display in the context drawer.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum ContextPage {
    #[default]
    About,
    Settings,
    Help,
}

fn theme_button_model() -> segmented_button::SingleSelectModel {
    segmented_button::SingleSelectModel::builder()
        .insert(|b| {
            b.text(fl!("theme-system"))
                .data(AppTheme::System)
                .activate()
        })
        .insert(|b| b.text(fl!("theme-dark")).data(AppTheme::Dark))
        .insert(|b| b.text(fl!("theme-light")).data(AppTheme::Light))
        .build()
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
        let settings_action = MenuAction::Settings;
        assert!(matches!(
            settings_action.message(),
            Message::ToggleContextPage(ContextPage::Settings)
        ));
    }
}
