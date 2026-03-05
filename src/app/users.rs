pub(crate) use crate::accounts_dbus::{AccountsProxyBlocking, UserProxyBlocking};
use cosmic::widget::{icon, nav_bar};
use nix::unistd::{Uid, User};
use std::sync::Arc;

/// Uses DBus synchronously to initialize users
pub fn initialize_users() -> (Vec<UserOption>, nav_bar::Model, Option<UserOption>) {
    let mut users = Vec::new();

    if let Ok(conn) = zbus::blocking::Connection::system()
        && let Ok(accounts) = AccountsProxyBlocking::new(&conn)
        && let Ok(user_paths) = accounts.list_cached_users()
    {
        for path in user_paths {
            if let Ok(builder) = UserProxyBlocking::builder(&conn).path(&path)
                && let Ok(user_proxy) = builder.build()
                && let (Ok(name), Ok(real_name), Ok(icon)) = (
                    user_proxy.user_name(),
                    user_proxy.real_name(),
                    user_proxy.icon_file(),
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

    // TODO: to use actual icon need custom nav
    let mut nav = nav_bar::Model::default();
    let mut selected_user = None;
    let current_username = User::from_uid(Uid::current())
        .ok()
        .flatten()
        .map(|u| u.name);

    // TODO: use actual icon
    for user_opt in &users {
        let id = nav
            .insert()
            .text(user_opt.to_string())
            .icon(icon::from_name("user-idle-symbolic"))
            .id();
        if selected_user.is_none() || current_username.as_deref() == Some(&*user_opt.username) {
            nav.activate(id);
            selected_user = Some(user_opt.clone());
        }
    }
    (users, nav, selected_user)
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
