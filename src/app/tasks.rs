// SPDX-License-Identifier: MPL-2.0

use crate::app::AppModel;
use crate::app::{error::AppError, fprint::*, message::{DeviceOption, Message}};
use crate::{fl, fprint_dbus::*};
use cosmic::{ApplicationExt, Task};

impl AppModel {
    /// Gets all registered prints for requested user
    pub(crate) fn list_fingers_task(&self) -> Task<cosmic::Action<Message>> {
        if let (Some(proxy), Some(user)) = (&self.device_proxy, &self.selected_user) {
            let proxy = proxy.clone();
            let username = (*user.username).clone();
            return Task::perform(
                async move {
                    match list_enrolled_fingers_dbus(proxy, username).await {
                        Ok(fingers) => Message::EnrolledFingers(fingers),
                        Err(e) => Message::OperationError(
                            AppError::from(e).with_context("Failed to list fingers"),
                        ),
                    }
                },
                cosmic::Action::App,
            );
        }
        Task::none()
    }

    /// Updates the header and window titles.
    pub fn update_title_task(&mut self) -> Task<cosmic::Action<Message>> {
        let mut window_title = fl!("app-title");

        if let Some(page) = self.nav.text(self.nav.active()) {
            window_title.push_str(" — ");
            window_title.push_str(page);
        }

        if let Some(id) = self.core.main_window_id() {
            self.set_window_title(window_title, id)
        } else {
            Task::none()
        }
    }
}

/// **Returns** ***Task*** which:
///
/// Request deletion of users all prints
pub fn task_delete_prints(
    path: zbus::zvariant::OwnedObjectPath,
    username: String,
    conn: zbus::Connection,
) -> Task<cosmic::Action<Message>> {
    Task::perform(
        async move {
            match delete_fingers(&conn, path, username).await {
                Ok(_) => Message::DeleteComplete(true),
                Err(e) => Message::OperationError(AppError::from(e)),
            }
        },
        cosmic::Action::App,
    )
}

pub fn get_devices_task(conn: zbus::Connection) -> Task<cosmic::Action<Message>> {
    Task::perform(
        async move {
            match find_all_devices(&conn).await {
                Ok(paths) => {
                    let mut devices = Vec::new();
                    for path in paths {
                        let name = match DeviceProxy::builder(&conn)
                            .path(path.clone())
                            .unwrap()
                            .build()
                            .await
                        {
                            Ok(proxy) => proxy.name().await.unwrap_or_else(|_| path.to_string()),
                            Err(_) => path.to_string(),
                        };
                        devices.push(DeviceOption { path, name });
                    }
                    Message::UpdateDevices(devices)
                }
                Err(e) => Message::OperationError(AppError::from(e)),
            }
        },
        cosmic::Action::App,
    )
}

/// **Returns** ***Task*** which:
///
/// Requests deletion of given users given print
pub fn task_delete_print(
    path: zbus::zvariant::OwnedObjectPath,
    username: String,
    finger_name: String,
    conn: zbus::Connection,
) -> Task<cosmic::Action<Message>> {
    Task::perform(
        async move {
            match delete_fingerprint_dbus(&conn, path, finger_name, username).await {
                Ok(_) => Message::DeleteComplete(false),
                Err(e) => Message::OperationError(AppError::from(e)),
            }
        },
        cosmic::Action::App,
    )
}

/// **Returns** ***Task*** which:
///
/// Sends a signal to stop current enroll process
pub fn task_enroll_stop(
    path: zbus::zvariant::OwnedObjectPath,
    conn: zbus::Connection,
) -> Task<cosmic::Action<Message>> {
    Task::perform(
        async move {
            let device = DeviceProxy::builder(&conn).path(path)?.build().await?;
            let _ = device.enroll_stop().await;
            device.release().await?;
            Ok::<(), zbus::Error>(())
        },
        |res| match res {
            Ok(_) => {
                cosmic::Action::App(Message::EnrollStatus("enroll-cancelled".to_string(), true))
            }
            Err(e) => cosmic::Action::App(Message::OperationError(AppError::from(e))),
        },
    )
}

/// **Returns** ***Task*** which:
///
/// Sends a signal to stop current verify process
pub fn task_verify_stop(
    path: zbus::zvariant::OwnedObjectPath,
    conn: zbus::Connection,
) -> Task<cosmic::Action<Message>> {
    Task::perform(
        async move {
            let device = DeviceProxy::builder(&conn).path(path)?.build().await?;
            let _ = device.verify_stop().await;
            device.release().await?;
            Ok::<(), zbus::Error>(())
        },
        |res| match res {
            Ok(_) => {
                cosmic::Action::App(Message::VerifyStatus("verify-cancelled".to_string(), true))
            }
            Err(e) => cosmic::Action::App(Message::OperationError(AppError::from(e))),
        },
    )
}

/// **Returns** ***Task*** which:
///
/// Requests deletion of all prints for all users
pub fn task_clear_device(
    path: zbus::zvariant::OwnedObjectPath,
    usernames: Vec<String>,
    conn: zbus::Connection,
) -> Task<cosmic::Action<Message>> {
    Task::perform(
        async move {
            match clear_all_fingers_dbus(&conn, path, usernames).await {
                Ok(_) => Message::ClearComplete(Ok(())),
                Err(e) => Message::ClearComplete(Err(AppError::from(e))),
            }
        },
        cosmic::Action::App,
    )
}

pub fn task_select_device(
    conn: zbus::Connection,
    path: zbus::zvariant::OwnedObjectPath,
) -> Task<cosmic::Action<Message>> {
    Task::perform(
        async move {
            match DeviceProxy::builder(&conn)
                .path(path.clone())
                .unwrap()
                .build()
                .await
            {
                Ok(proxy) => Message::DeviceFound(Some((path, proxy))),
                Err(e) => Message::OperationError(AppError::from(e)),
            }
        },
        cosmic::Action::App,
    )
}

/// **Returns** ***Task*** which:
/// Uses zbus to find and return default fingerprint scanner device
pub fn task_find_device(conn_clone: zbus::Connection) -> Task<cosmic::Action<Message>> {
    Task::perform(
        async move {
            match find_device(&conn_clone).await {
                Ok((path, proxy)) => Message::DeviceFound(Some((path, proxy))),
                Err(e) => {
                    let error = AppError::from(e);
                    if matches!(error, AppError::Unknown(_)) {
                        Message::OperationError(AppError::DeviceNotFound)
                    } else {
                        Message::OperationError(error)
                    }
                }
            }
        },
        cosmic::Action::App,
    )
}

/// **Returns** ***Task*** which:
/// Connects to DBus
pub fn task_connect() -> Task<cosmic::Action<Message>> {
    Task::perform(
        async move {
            match zbus::Connection::system().await {
                Ok(conn) => Message::ConnectionReady(conn),
                Err(e) => Message::OperationError(AppError::ConnectDbus(e.to_string())),
            }
        },
        cosmic::Action::App,
    )
}
