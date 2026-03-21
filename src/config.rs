// SPDX-License-Identifier: MPL-2.0
use cosmic::cosmic_config::{self, CosmicConfigEntry, cosmic_config_derive::CosmicConfigEntry};
use cosmic::{Theme, theme};
use serde::{Deserialize, Serialize};

// AppTheme is directly copied from https://github.com/cosmic-utils/camera
/// Application theme preference
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Hash)]
pub enum AppTheme {
    /// Follow system theme (dark or light based on system setting)
    #[default]
    System,
    /// Always use dark theme
    Dark,
    /// Always use light theme
    Light,
}

impl AppTheme {
    /// Get the COSMIC theme for this app theme preference.
    ///
    /// On non-COSMIC desktops, `system_dark()`/`system_light()`/`system_preference()`
    /// read broken defaults from cosmic_config, so we use built-in themes instead.
    /// For `System` mode, the initial theme defaults to dark; the portal subscription
    /// in `mod.rs` sends the correct value asynchronously once connected.
    pub fn theme(&self) -> Theme {
        if is_cosmic_desktop() {
            match self {
                Self::Dark => {
                    let mut t = theme::system_dark();
                    t.theme_type.prefer_dark(Some(true));
                    t
                }
                Self::Light => {
                    let mut t = theme::system_light();
                    t.theme_type.prefer_dark(Some(false));
                    t
                }
                Self::System => theme::system_preference(),
            }
        } else {
            match self {
                Self::Dark | Self::System => Theme::dark(),
                Self::Light => Theme::light(),
            }
        }
    }
}

#[derive(Debug, Default, Clone, CosmicConfigEntry, Eq, PartialEq)]
#[version = 2]
pub struct Config {
    pub app_theme: AppTheme,
    pub experimental_ui: bool,
}

/// Whether we're running on the COSMIC desktop (cached for process lifetime).
pub fn is_cosmic_desktop() -> bool {
    static IS_COSMIC: std::sync::LazyLock<bool> = std::sync::LazyLock::new(|| {
        std::env::var("XDG_CURRENT_DESKTOP")
            .map(|d| d.to_ascii_uppercase().contains("COSMIC"))
            .unwrap_or(false)
    });
    *IS_COSMIC
}
