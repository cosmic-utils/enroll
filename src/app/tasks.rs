// SPDX-License-Identifier: MPL-2.0

use crate::app::{error::AppError, fprint::*, message::Message};
use crate::fprint_dbus::*;
use cosmic::Task;

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
