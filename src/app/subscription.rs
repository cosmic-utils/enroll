use crate::app::{
    Message,
    error::AppError,
    fprint::{enroll_fingerprint_process, verify_finger_process},
};
use ashpd::desktop::settings::{ColorScheme, Settings};
use cosmic::iced::{Subscription, futures::channel::mpsc::Sender, stream::channel};
use futures_util::{SinkExt, StreamExt};

#[derive(Clone)]
pub(crate) struct VerifyData {
    device_path: std::sync::Arc<zbus::zvariant::OwnedObjectPath>,
    connection: zbus::Connection,
    username: std::sync::Arc<String>,
    finger: String,
}

impl VerifyData {
    pub(crate) fn new(
        device_path: std::sync::Arc<zbus::zvariant::OwnedObjectPath>,
        connection: zbus::Connection,
        username: std::sync::Arc<String>,
        finger: String,
    ) -> Self {
        Self {
            device_path,
            connection,
            username,
            finger,
        }
    }
}

impl std::hash::Hash for VerifyData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.username.hash(state);
        self.finger.hash(state);
    }
}

#[derive(Clone)]
pub(crate) struct EnrollData {
    finger_name: std::sync::Arc<String>,
    device_path: std::sync::Arc<zbus::zvariant::OwnedObjectPath>,
    connection: zbus::Connection,
    username: std::sync::Arc<String>,
}

impl EnrollData {
    pub(crate) fn new(
        finger_name: std::sync::Arc<String>,
        device_path: std::sync::Arc<zbus::zvariant::OwnedObjectPath>,
        connection: zbus::Connection,
        username: std::sync::Arc<String>,
    ) -> Self {
        Self {
            finger_name,
            device_path,
            connection,
            username,
        }
    }
}

impl std::hash::Hash for EnrollData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.finger_name.hash(state);
        self.username.hash(state);
    }
}

/// ***Returns*** a subscription to an ongoing enroll process
pub(crate) fn enroll_subscription(data: EnrollData) -> Subscription<Message> {
    Subscription::run_with(data, |data| {
        let data = data.clone();
        channel(100, move |mut output: Sender<Message>| async move {
            // Implement enrollment stream here
            match enroll_fingerprint_process(
                data.connection,
                &data.device_path,
                &data.finger_name,
                &data.username,
                &mut output,
            )
            .await
            {
                Ok(_) => {}
                Err(e) => {
                    let _ = output
                        .send(Message::OperationError(AppError::from(e)))
                        .await;
                }
            }
            futures_util::future::pending().await
        })
    })
}

/// ***Returns*** a subscription to an ongoing verify process
pub(crate) fn verify_subscription(data: VerifyData) -> Subscription<Message> {
    Subscription::run_with(data, |data| {
        let data = data.clone();
        channel(100, move |mut output: Sender<Message>| async move {
            let path = (*data.device_path).clone();
            let username = (*data.username).clone();

            match verify_finger_process(&data.connection, path, data.finger, username, &mut output)
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    let _ = output
                        .send(Message::OperationError(AppError::from(e)))
                        .await;
                }
            }
            futures_util::future::pending().await
        })
    })
}

// On non-COSMIC desktops, subscribe to XDG portal color-scheme changes
// so theme updates when user changes their desktop appearance
pub fn portal_theme_subscription(app_theme: crate::config::AppTheme) -> Subscription<Message> {
    if !crate::config::is_cosmic_desktop() && app_theme == crate::config::AppTheme::System {
        Subscription::run_with(app_theme, |_| {
            channel(10, async move |mut output: Sender<Message>| {
                let Ok(settings) = Settings::new().await else {
                    tracing::warn!("Failed to create XDG Settings portal proxy");
                    std::future::pending::<()>().await;
                    return;
                };

                let send_scheme =
                    |output: &mut cosmic::iced::futures::channel::mpsc::Sender<Message>,
                     scheme: ColorScheme| {
                        let is_dark = !matches!(scheme, ColorScheme::PreferLight);
                        output.try_send(Message::ThemeChanged(is_dark)).ok();
                    };

                // Send initial color scheme
                if let Ok(scheme) = settings.color_scheme().await {
                    send_scheme(&mut output, scheme);
                }

                // Subscribe to live changes via ashpd's D-Bus signal stream
                if let Ok(mut stream) = settings.receive_color_scheme_changed().await {
                    while let Some(scheme) = StreamExt::next(&mut stream).await {
                        send_scheme(&mut output, scheme);
                    }
                }

                tracing::warn!("Portal color-scheme stream ended");
                std::future::pending::<()>().await;
            })
        })
    } else {
        Subscription::none()
    }
}
