// SPDX-License-Identifier: MPL-2.0

pub(crate) use crate::accounts_dbus::{AccountsProxy, UserProxy};
use cosmic::widget::{icon, nav_bar};
use nix::unistd::{Uid, User};
use std::sync::Arc;

/// Fetches users from accounts-daemon asynchronously.
///
/// **Returns** list of users
pub async fn fetch_users() -> Vec<UserOption> {
    let mut users = Vec::new();

    if let Ok(conn) = zbus::Connection::system().await
        && let Ok(accounts) = AccountsProxy::new(&conn).await
        && let Ok(user_paths) = accounts.list_cached_users().await
    {
        for path in user_paths {
            if let Ok(builder) = UserProxy::builder(&conn).path(&path)
                && let Ok(user_proxy) = builder.build().await
                && let (Ok(name), Ok(real_name), Ok(icon)) = (
                    user_proxy.user_name().await,
                    user_proxy.real_name().await,
                    user_proxy.icon_file().await,
                )
            {
                users.push(UserOption {
                    username: Arc::new(name),
                    realname: Arc::new(real_name),
                    icon: Arc::new(icon),
                });
            }
        }
    }

    users
}

/// Builds the nav bar model from a list of users and selects the current user.
///
/// **Returns** tuple of nav model and selected user
pub fn build_nav(users: &[UserOption]) -> (nav_bar::Model, Option<UserOption>) {
    let mut nav = nav_bar::Model::default();
    let mut selected_user = None;
    let current_username = User::from_uid(Uid::current())
        .ok()
        .flatten()
        .map(|u| u.name);

    for user_opt in users {
        let mut item = nav.insert().text(user_opt.to_string());
        let mut icon_str = user_opt.icon.as_str();

        if icon_str.starts_with("file://") {
            icon_str = &icon_str[7..];
        }

        let icon: cosmic::widget::Icon = if icon_str.is_empty() {
            icon::from_name("user-idle-symbolic").into()
        } else if icon_str.contains('/') {
            let path = std::path::PathBuf::from(icon_str);
            if path.exists() {
                icon::icon(icon::from_path(path)).size(24)
            } else {
                tracing::warn!("User icon path does not exist: {}", icon_str);
                icon::from_name("user-idle-symbolic").into()
            }
        } else {
            icon::from_name(icon_str).into()
        };

        item = item.icon(icon);
        let id = item.id();
        if selected_user.is_none() || current_username.as_deref() == Some(&*user_opt.username) {
            nav.activate(id);
            selected_user = Some(user_opt.clone());
        }
    }
    (nav, selected_user)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserOption {
    pub username: Arc<String>,
    pub realname: Arc<String>,
    pub icon: Arc<String>,
}

impl std::fmt::Display for UserOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.realname.is_empty() {
            write!(f, "{}", self.username)
        } else {
            write!(f, "{} ({})", self.realname, self.username)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_user_option_display_with_realname() {
        let user_option = UserOption {
            username: Arc::new("jdoe".to_string()),
            realname: Arc::new("John Doe".to_string()),
            icon: Arc::new("".to_string()),
        };
        assert_eq!(user_option.to_string(), "John Doe (jdoe)");
    }

    #[test]
    fn test_user_option_display_without_realname() {
        let user_option = UserOption {
            username: Arc::new("jdoe".to_string()),
            realname: Arc::new("".to_string()),
            icon: Arc::new("".to_string()),
        };
        assert_eq!(user_option.to_string(), "jdoe");
    }

    #[test]
    fn test_user_option_display_with_whitespace_realname() {
        let user_option = UserOption {
            username: Arc::new("jdoe".to_string()),
            realname: Arc::new("   ".to_string()),
            icon: Arc::new("".to_string()),
        };
        assert_eq!(user_option.to_string(), "    (jdoe)");
    }

    #[test]
    fn test_user_option_display_empty_username() {
        let user_option = UserOption {
            username: Arc::new("".to_string()),
            realname: Arc::new("John Doe".to_string()),
            icon: Arc::new("".to_string()),
        };
        assert_eq!(user_option.to_string(), "John Doe ()");
    }

    #[test]
    fn test_user_option_display_both_empty() {
        let user_option = UserOption {
            username: Arc::new("".to_string()),
            realname: Arc::new("".to_string()),
            icon: Arc::new("".to_string()),
        };
        assert_eq!(user_option.to_string(), "");
    }
}
