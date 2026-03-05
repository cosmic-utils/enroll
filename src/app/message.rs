// SPDX-License-Identifier: MPL-2.0

use crate::app::ContextPage;
use crate::app::error::AppError;
use crate::config::Config;
use crate::fprint_dbus::DeviceProxy;

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
    DeleteComplete,
    ClearDevice,
    CancelClear,
    ClearComplete(Result<(), AppError>),
    EnrolledFingers(Vec<String>),
    FingerSelected(String),
}
