// SPDX-License-Identifier: MPL-2.0

use crate::app::error::AppError;
use crate::app::message::Message;
use crate::fprint_dbus::{DeviceProxy, ManagerProxy};
use futures_util::sink::Sink;
use futures_util::{SinkExt, StreamExt};

pub async fn find_device(
    connection: &zbus::Connection,
) -> zbus::Result<(zbus::zvariant::OwnedObjectPath, DeviceProxy<'static>)> {
    let manager = ManagerProxy::new(connection).await?;
    let path = manager.get_default_device().await?;
    let device = DeviceProxy::builder(connection)
        .path(path.clone())?
        .build()
        .await?;
    Ok((path, device))
}

/// fprintd DBus API function for requesting users registered prints
/// # Return
/// Array containing all users registered fingerprints as strings
/// # Errors
/// net.reactivated.Fprint.Error.PermissionDenied:
/// if the caller lacks the appropriate PolicyKit authorization
/// net.reactivated.Fprint.Error.NoEnrolledPrints:
/// if the chosen user doesn't have any fingerprints enrolled
pub async fn list_enrolled_fingers_dbus(
    device: &DeviceProxy<'static>,
    username: String,
) -> zbus::Result<Vec<String>> {
    validate_username(&username)?;
    device.list_enrolled_fingers(&username).await
}

pub async fn delete_fingerprint_dbus(
    connection: &zbus::Connection,
    path: zbus::zvariant::OwnedObjectPath,
    finger: String,
    username: String,
) -> zbus::Result<()> {
    validate_username(&username)?;
    let device = DeviceProxy::builder(connection).path(path)?.build().await?;

    device.claim(&username).await?;
    let res = device.delete_enrolled_finger(&finger).await;
    let rel_res = device.release().await;
    res.and(rel_res)
}

pub async fn delete_fingers(
    connection: &zbus::Connection,
    path: zbus::zvariant::OwnedObjectPath,
    username: String,
) -> zbus::Result<()> {
    validate_username(&username)?;
    let device = DeviceProxy::builder(connection).path(path)?.build().await?;

    device.claim(&username).await?;
    let _ = device.delete_enrolled_fingers2().await;
    device.release().await
}

pub async fn clear_all_fingers_dbus(
    connection: &zbus::Connection,
    path: zbus::zvariant::OwnedObjectPath,
    usernames: Vec<String>,
) -> zbus::Result<()> {
    let device = DeviceProxy::builder(connection).path(path)?.build().await?;
    let mut last_error = None;

    for username in usernames {
        if let Err(e) = validate_username(&username) {
            last_error = Some(e);
            continue;
        }

        if let Err(e) = device.claim(&username).await {
            last_error = Some(e);
            continue;
        }

        match device.list_enrolled_fingers(&username).await {
            Ok(fingers) => {
                for finger in fingers {
                    if let Err(e) = device.delete_enrolled_finger(&finger).await {
                        last_error = Some(e);
                    }
                }
            }
            Err(e) => {
                last_error = Some(e);
            }
        }

        if let Err(e) = device.release().await {
            last_error = Some(e);
        }
    }

    if let Some(e) = last_error {
        Err(e)
    } else {
        Ok(())
    }
}

pub async fn enroll_fingerprint_process<S>(
    connection: zbus::Connection,
    path: &zbus::zvariant::OwnedObjectPath,
    finger_name: &str,
    username: &str,
    output: &mut S,
) -> zbus::Result<()>
where
    S: Sink<Message> + Unpin + Send,
    S::Error: std::fmt::Debug + Send,
{
    validate_username(username)?;
    let device = DeviceProxy::builder(&connection)
        .path(path)?
        .build()
        .await?;

    // Claim device
    match device.claim(username).await {
        Ok(_) => {}
        Err(e) => return Err(e),
    };

    let total_stages = match device.num_enroll_stages().await {
        Ok(n) if n > 0 => Some(n as u32),
        _ => None,
    };
    let _ = output.send(Message::EnrollStart(total_stages)).await;

    // Start enrollment
    if let Err(e) = device.enroll_start(finger_name).await {
        let _ = device.release().await;
        return Err(e);
    }

    // Listen for signals
    let mut stream = match device.receive_enroll_status().await {
        Ok(s) => s,
        Err(e) => {
            let _ = device.release().await;
            return Err(e);
        }
    };

    while let Some(signal) = stream.next().await {
        let args = signal.args();
        match args {
            Ok(args) => {
                let result: String = args.result;
                let done: bool = args.done;

                // Map result string to user friendly message if needed, or pass through
                let _ = output.send(Message::EnrollStatus(result, done)).await;

                if done {
                    break;
                }
            }
            Err(_) => {
                let _ = output
                    .send(Message::OperationError(AppError::Unknown(
                        "Failed to parse signal".to_string(),
                    )))
                    .await;
                break;
            }
        }
    }

    // Release device
    let _ = device.release().await;

    Ok(())
}

/// Request via DBus for the users fingerprint to be verified.
///
/// # Errors
///net.reactivated.Fprint.Error.PermissionDenied:
/// if the caller lacks the appropriate PolicyKit authorization
/// net.reactivated.Fprint.Error.ClaimDevice:
/// if the device was not claimed
/// net.reactivated.Fprint.Error.AlreadyInUse:
/// if the device was already being used
/// net.reactivated.Fprint.Error.NoActionInProgress:
/// if there was no ongoing verification
/// net.reactivated.Fprint.Error.NoEnrolledPrints:
/// if there are no enrolled prints for the chosen user
/// net.reactivated.Fprint.Error.Internal:
/// if there was an internal error
pub async fn verify_finger_dbus(
    connection: &zbus::Connection,
    path: zbus::zvariant::OwnedObjectPath,
    finger: String,
    username: String,
) -> zbus::Result<()> {
    validate_username(&username)?;
    let device = DeviceProxy::builder(connection).path(path)?.build().await?;

    device.claim(&username).await?;

    let mut status_stream = match device.receive_verify_status().await {
        Ok(s) => s,
        Err(e) => {
            let _ = device.release().await;
            return Err(e);
        }
    };

    if let Err(e) = device.verify_start(&finger).await {
        let _ = device.release().await;
        return Err(e);
    }

    // TODO: send reference to self and implement Message::VerifyStatus(String)
    while let Some(signal) = status_stream.next().await {
        match signal.args() {
            Ok(args) => {
                let result: String = args.result;
                let done: bool = args.done;
                if done {
                    break;
                }
            }
            Err(e) => {
                break;
            }
        }
    }

    let _ = device.verify_stop().await;
    device.release().await
}

fn validate_username(username: &str) -> zbus::Result<()> {
    if username.is_empty() {
        return Err(zbus::Error::Failure("Username cannot be empty".to_string()));
    }
    if username.len() > 255 {
        return Err(zbus::Error::Failure("Username is too long".to_string()));
    }
    if !username
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.')
    {
        return Err(zbus::Error::Failure(format!(
            "Invalid characters in username: {}",
            username
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_username() {
        // Valid usernames
        assert!(validate_username("user").is_ok());
        assert!(validate_username("user1").is_ok());
        assert!(validate_username("user_name").is_ok());
        assert!(validate_username("user-name").is_ok());
        assert!(validate_username("user.name").is_ok());
        assert!(validate_username("u").is_ok());
        assert!(validate_username("123").is_ok());
        assert!(validate_username("User").is_ok()); // Uppercase is allowed by our validation

        // Invalid usernames
        assert!(validate_username("").is_err());
        assert!(validate_username("user name").is_err()); // space
        assert!(validate_username("user/name").is_err()); // slash
        assert!(validate_username("user@name").is_err()); // @
        assert!(validate_username("user!name").is_err()); // !
        assert!(validate_username("user?name").is_err()); // ?

        let long_name = "a".repeat(256);
        assert!(validate_username(&long_name).is_err());

        let max_len_name = "a".repeat(255);
        assert!(validate_username(&max_len_name).is_ok());
    }
}
