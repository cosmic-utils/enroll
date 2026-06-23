// SPDX-License-Identifier: MPL-2.0

use crate::app::error::AppError;
use crate::app::message::Message;
use crate::fprint_dbus::{DeviceProxy, ManagerProxy};
use futures_util::sink::Sink;
use futures_util::{SinkExt, StreamExt};

/// **Returns** the default fingerprint reader device.
/// *device:*
/// The object path for the default device.
/// # Errors
/// ***net.reactivated.Fprint.Error.NoSuchDevice:***
/// if the device does not exist
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

pub async fn find_all_devices(
    connection: &zbus::Connection,
) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>> {
    let manager = ManagerProxy::new(connection).await?;
    Ok(manager.get_devices().await?)
}

/// fprintd DBus API function for requesting users registered prints
/// # Return
/// Array containing all users registered fingerprints as strings
/// # Errors
/// ***net.reactivated.Fprint.Error.PermissionDenied:***
/// if the caller lacks the appropriate PolicyKit authorization
/// ***net.reactivated.Fprint.Error.NoEnrolledPrints:***
/// if the chosen user doesn't have any fingerprints enrolled
/// ***net.reactivated.Fprint.Error.AlreadyInUse:***
/// if the device is already claimed
/// ***net.reactivated.Fprint.Error.Internal:***
/// if the device couldn't be claimed
pub async fn list_enrolled_fingers_dbus(
    device: DeviceProxy<'static>,
    username: String,
) -> zbus::Result<Vec<String>> {
    validate_username(&username)?;
    device.list_enrolled_fingers(&username).await
}

/// Returns true when the error means the running fprintd implementation does
/// not provide the requested method (e.g. open-fprintd only implements the
/// legacy `DeleteEnrolledFingers`).
fn is_unsupported(err: &zbus::Error) -> bool {
    AppError::from(err.clone()) == AppError::UnsupportedOperation
}

/// Deletes every enrolled fingerprint for a user, using the modern
/// `DeleteEnrolledFingers2` when available and falling back to the legacy
/// `DeleteEnrolledFingers(username)` for daemons like open-fprintd.
async fn delete_all_fingers(device: &DeviceProxy<'_>, username: &str) -> zbus::Result<()> {
    match device.delete_enrolled_fingers2().await {
        Ok(()) => Ok(()),
        Err(e) if is_unsupported(&e) => device.delete_enrolled_fingers(username).await,
        Err(e) => Err(e),
    }
}

/// Deletes chosen fingers print record for single user
/// # Returns
/// Ok()
/// # Errors
/// ***net.reactivated.Fprint.Error.PermissionDenied:***
/// if the caller lacks the appropriate PolicyKit authorization
/// ***net.reactivated.Fprint.Error.PrintsNotDeleted:***
/// if the fingerprint is not deleted from fprintd storage
/// ***net.reactivated.Fprint.Error.AlreadyInUse:***
/// if the device is already claimed
/// ***net.reactivated.Fprint.Error.Internal:***
/// if the device couldn't be claimed
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

/// Deletes all print records for chosen user
/// # Returns
/// Ok()
/// # Errors
/// ***net.reactivated.Fprint.Error.PermissionDenied:***
/// if the caller lacks the appropriate PolicyKit authorization
/// ***net.reactivated.Fprint.Error.PrintsNotDeleted:***
/// if the fingerprint is not deleted from fprintd storage
/// ***net.reactivated.Fprint.Error.AlreadyInUse:***
/// if the device is already claimed
/// ***net.reactivated.Fprint.Error.Internal:***
/// if the device couldn't be claimed
pub async fn delete_fingers(
    connection: &zbus::Connection,
    path: zbus::zvariant::OwnedObjectPath,
    username: String,
) -> zbus::Result<()> {
    validate_username(&username)?;
    let device = DeviceProxy::builder(connection).path(path)?.build().await?;

    device.claim(&username).await?;
    let res = delete_all_fingers(&device, &username).await;
    let rel_res = device.release().await;
    res.and(rel_res)
}

/// Deletes all prints for all currently known users
/// # Returns
/// Ok()
/// # Errors
/// ***net.reactivated.Fprint.Error.PermissionDenied:***
/// if the caller lacks the appropriate PolicyKit authorization
/// ***net.reactivated.Fprint.Error.PrintsNotDeleted:***
/// if the fingerprint is not deleted from fprintd storage
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
                    match device.delete_enrolled_finger(&finger).await {
                        Ok(()) => {}
                        Err(e) if is_unsupported(&e) => {
                            // Legacy daemon has no per-finger delete; remove all at once.
                            if let Err(e) = device.delete_enrolled_fingers(&username).await {
                                last_error = Some(e);
                            }
                            break;
                        }
                        Err(e) => last_error = Some(e),
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

/// Records a print into scanner devices. Does it by communicating via
/// the net.reactived.Fprintd API with the device.
///
/// Updates status of the app through a Subscription.
///
/// # Returns
/// Result(Ok(). Or Result(zbus::Error()))
/// # Errors
/// ***net.reactivated.Fprint.Error.PermissionDenied:***
/// if the caller lacks the appropriate PolicyKit authorization
/// ***net.reactivated.Fprint.Error.ClaimDevice:***
/// if the device was not claimed
/// ***net.reactivated.Fprint.Error.AlreadyInUse:***
/// if the device was already being used
/// ***net.reactivated.Fprint.Error.InvalidFingername:***
/// if the finger name passed is invalid
/// ***net.reactivated.Fprint.Error.Internal:***
/// if there was an internal error
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
    device.claim(username).await?;

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
/// ***net.reactivated.Fprint.Error.PermissionDenied:***
/// if the caller lacks the appropriate PolicyKit authorization
/// ***net.reactivated.Fprint.Error.ClaimDevice:***
/// if the device was not claimed
/// ***net.reactivated.Fprint.Error.AlreadyInUse:***
/// if the device was already being used
/// ***net.reactivated.Fprint.Error.NoActionInProgress:***
/// if there was no ongoing verification
/// ***net.reactivated.Fprint.Error.NoEnrolledPrints:***
/// if there are no enrolled prints for the chosen user
/// ***net.reactivated.Fprint.Error.Internal:***
/// if there was an internal error
pub async fn verify_finger_process<S>(
    connection: zbus::Connection,
    path: &zbus::zvariant::OwnedObjectPath,
    finger: &str,
    username: &str,
    output: &mut S,
) -> zbus::Result<()>
where
    S: Sink<Message> + Unpin + Send,
    S::Error: std::fmt::Debug + Send,
{
    validate_username(&username)?;
    let device = DeviceProxy::builder(&connection)
        .path(path)?
        .build()
        .await?;

    device.claim(&username).await?;

    if let Err(e) = device.verify_start(&finger).await {
        let _ = device.release().await;
        return Err(e);
    }

    let mut status_stream = match device.receive_verify_status().await {
        Ok(s) => s,
        Err(e) => {
            let _ = device.release().await;
            return Err(e);
        }
    };

    while let Some(signal) = status_stream.next().await {
        match signal.args() {
            Ok(args) => {
                let result: String = args.result;
                let done: bool = args.done;

                let _ = output.send(Message::VerifyStatus(result, done)).await;

                if done {
                    break;
                }
            }
            Err(_e) => {
                let _ = output
                    .send(Message::OperationError(AppError::Unknown(
                        "Failed to parse signal".to_string(),
                    )))
                    .await;
                break;
            }
        }
    }

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

    #[test]
    fn test_is_unsupported() {
        use zbus::message::Message;
        use zbus::names::ErrorName;

        fn method_error(name: &str) -> zbus::Error {
            let msg = Message::method_call("/", "Ping")
                .unwrap()
                .destination("org.freedesktop.DBus")
                .unwrap()
                .build(&())
                .unwrap();
            let error_name = ErrorName::try_from(name).unwrap();
            zbus::Error::MethodError(error_name.into(), None, msg)
        }

        // Missing method on the daemon (e.g. open-fprintd) is "unsupported".
        assert!(is_unsupported(&method_error(
            "org.freedesktop.DBus.Error.UnknownMethod"
        )));
        // A normal fprintd error must not be treated as unsupported.
        assert!(!is_unsupported(&method_error(
            "net.reactivated.Fprint.Error.PermissionDenied"
        )));
    }
}
