// SPDX-License-Identifier: MPL-2.0
use crate::accounts_dbus::{AccountsProxy, UserProxy};
use crate::app::MenuAction;
use crate::app::error::*;
use crate::app::finger::*;
use crate::app::fprint::*;
use crate::app::message::*;
use crate::config::Config;
use crate::fl;
use crate::fprint_dbus::DeviceProxy;

use cosmic::app::context_drawer;
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::widget::pick_list;
use cosmic::iced::{Alignment, Length, Subscription};
use cosmic::prelude::*;
use cosmic::widget::{self, dialog, menu, nav_bar, text};
use cosmic::{cosmic_theme, theme};
use futures_util::SinkExt;
use futures_util::stream::{self, StreamExt};
use nix::unistd::{Uid, User};
use std::collections::HashMap;
use std::sync::Arc;

const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const APP_ICON: &[u8] = include_bytes!("../../resources/icons/hicolor/scalable/apps/enroll.svg");
const FPRINT_ICON: &[u8] = include_bytes!("../../resources/icons/hicolor/scalable/apps/fprint.svg");

const STATUS_TEXT_SIZE: u16 = 16;
const PROGRESS_BAR_HEIGHT: u16 = 10;
const MAIN_SPACING: u16 = 20;
const MAIN_PADDING: u16 = 20;

const USER_FETCH_CONCURRENCY: usize = 10;
use super::AppModel;

/// Create a COSMIC application from the app model
impl cosmic::Application for AppModel {
    /// The async executor that will be used to run your application's commands.
    type Executor = cosmic::executor::Default;

    /// Data that your application receives to its init method.
    type Flags = ();

    /// Messages which the application and its widgets will emit.
    type Message = Message;

    /// Unique identifier in RDNN (reverse domain name notation) format.
    const APP_ID: &'static str = "org.cosmic_utils.Enroll";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    /// Initializes the application with any given flags and startup commands.
    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        // Get current user
        let user = User::from_uid(Uid::current())
            .ok()
            .flatten()
            .map(|u| UserOption {
                username: Arc::new(u.name),
                realname: Arc::new(u.gecos.to_string_lossy().into_owned()),
                icon: Arc::new("".to_string()),
            });

        // Construct the app model with the runtime's core.
        let mut app = AppModel {
            core,
            context_page: ContextPage::default(),
            nav: nav_bar::Model::default(),
            key_binds: HashMap::new(),
            config: Config::default(),
            status: fl!("status-connecting"),
            device_path: None,
            device_proxy: None,
            connection: None,
            busy: true,
            enrolling_finger: None,
            enroll_progress: 0,
            enroll_total_stages: None,
            users: Vec::new(),
            selected_user: user,
            selected_finger: Finger::default(),
            enrolled_fingers: Vec::new(),
            confirm_clear: false,
        };

        // Create a startup command that sets the window title.
        let command = app.update_title();

        // Start async task to connect to DBus
        let connect_task = Task::perform(
            async move {
                match zbus::Connection::system().await {
                    Ok(conn) => Message::ConnectionReady(conn),
                    Err(e) => Message::OperationError(AppError::ConnectDbus(e.to_string())),
                }
            },
            cosmic::Action::App,
        );

        let config_task = Task::perform(
            async move {
                let config = tokio::task::spawn_blocking(move || {
                    cosmic_config::Config::new(Self::APP_ID, Config::VERSION)
                        .map(|context| match Config::get_entry(&context) {
                            Ok(config) => config,
                            Err((errors, config)) => {
                                for why in errors {
                                    tracing::error!(%why, "error loading app config");
                                }

                                config
                            }
                        })
                        .unwrap_or_default()
                })
                .await
                .unwrap_or_else(|e| {
                    tracing::error!("Config task join error: {}", e);
                    Config::default()
                });

                Message::UpdateConfig(config)
            },
            cosmic::Action::App,
        );

        (app, Task::batch(vec![command, connect_task, config_task]))
    }

    /// Elements to pack at the start of the header bar.
    fn header_start(&self) -> Vec<Element<'_, Self::Message>> {
        let menu_bar = menu::bar(vec![menu::Tree::with_children(
            Element::from(menu::root(fl!("view"))),
            menu::items(
                &self.key_binds,
                vec![menu::Item::Button(fl!("about"), None, MenuAction::About)],
            ),
        )]);

        vec![menu_bar.into()]
    }

    /// Enables the COSMIC application to create a nav bar with this model.
    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav)
    }

    /// Display a context drawer if the context page is requested.
    fn context_drawer(&self) -> Option<context_drawer::ContextDrawer<'_, Self::Message>> {
        if !self.core.window.show_context {
            return None;
        }

        Some(match self.context_page {
            ContextPage::About => context_drawer::context_drawer(
                self.about(),
                Message::ToggleContextPage(ContextPage::About),
            )
            .title(fl!("about")),
        })
    }

    /// Display a dialog in the center of the application window when `Some`.
    fn dialog(&self) -> Option<Element<'_, Self::Message>> {
        if self.confirm_clear {
            Some(
                dialog::dialog()
                    .title(fl!("clear-device"))
                    .body(fl!("clear-device-confirm"))
                    .primary_action(
                        widget::button::destructive(fl!("clear-device"))
                            .on_press(Message::ClearDevice),
                    )
                    .secondary_action(
                        widget::button::standard(fl!("cancel")).on_press(Message::CancelClear),
                    )
                    .into(),
            )
        } else {
            None
        }
    }

    /// Describes the interface based on the current state of the application model.
    ///
    /// Application events will be processed through the view. Any messages emitted by
    /// events received by widgets will be passed to the update method.
    fn view(&self) -> Element<'_, Self::Message> {
        let mut column = widget::column();

        if let Some(picker) = self.view_finger_picker() {
            column = column.push(picker);
        }

        column = column.push(self.view_icon()).push(self.view_status());

        if let Some(progress) = self.view_progress() {
            column = column.push(progress);
        }

        column
            .push(self.view_controls())
            .align_x(Horizontal::Center)
            .spacing(MAIN_SPACING)
            .padding(MAIN_PADDING)
            .into()
    }

    /// Register subscriptions for this application.
    ///
    /// Subscriptions are long-running async tasks running in the background which
    /// emit messages to the application through a channel. They are started at the
    /// beginning of the application, and persist through its lifetime.
    fn subscription(&self) -> Subscription<Self::Message> {
        struct MySubscription;
        struct EnrollmentSubscription;

        let mut subscriptions = vec![
            // Create a subscription which emits updates through a channel.
            Subscription::run_with_id(
                std::any::TypeId::of::<MySubscription>(),
                cosmic::iced::stream::channel(4, move |_channel| async move {
                    futures_util::future::pending().await
                }),
            ),
            // Watch for application configuration changes.
            self.core()
                .watch_config::<Config>(Self::APP_ID)
                .map(|update| {
                    for why in update.errors {
                        tracing::error!(?why, "app config error");
                    }

                    Message::UpdateConfig(update.config)
                }),
        ];

        // Add enrollment subscription if enrolling
        if let (Some(finger_name), Some(device_path), Some(connection), Some(user)) = (
            &self.enrolling_finger,
            &self.device_path,
            &self.connection,
            &self.selected_user,
        ) {
            let finger_name = finger_name.clone();
            let device_path = device_path.clone();
            let connection = connection.clone();
            let user = user.clone();

            subscriptions.push(Subscription::run_with_id(
                std::any::TypeId::of::<EnrollmentSubscription>(),
                cosmic::iced::stream::channel(100, move |mut output| async move {
                    // Implement enrollment stream here
                    match enroll_fingerprint_process(
                        connection,
                        &device_path,
                        &finger_name,
                        &user.username,
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
                }),
            ));
        }

        Subscription::batch(subscriptions)
    }

    /// Handles messages emitted by the application and its widgets.
    ///
    /// Tasks may be returned for asynchronous execution of code in the background
    /// on the application's async runtime.
    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::ConnectionReady(conn) => self.on_connection_ready(conn),
            Message::UsersFound(users) => self.on_users_found(users),
            Message::FingerSelected(finger) => self.on_finger_selected(finger),
            Message::DeviceFound(path) => self.on_device_found(path),
            Message::EnrolledFingers(fingers) => self.on_fingers_listed(fingers),
            Message::OperationError(err) => self.on_error(err),
            Message::EnrollStart(total) => self.on_enroll_start(total),
            Message::EnrollStatus(status, done) => self.on_enroll_status(status, done),
            Message::EnrollStop => self.on_enroll_stop(),
            Message::DeleteComplete => self.on_delete_complete(),
            Message::Delete => self.on_delete(),
            Message::ClearDevice => self.on_clear_device(),
            Message::CancelClear => self.on_cancel_clear(),
            Message::ClearComplete(res) => self.on_clear_completion(res),
            Message::Register => self.on_register(),
            Message::OpenRepositoryUrl => self.on_clicked_link(),
            Message::ToggleContextPage(context_page) => self.on_context_page_toggle(context_page),
            Message::UpdateConfig(config) => self.on_update_config(config),
            Message::LaunchUrl(url) => self.on_open_link(url),
        }
    }

    /// Called when a nav item is selected.
    fn on_nav_select(&mut self, id: nav_bar::Id) -> Task<cosmic::Action<Self::Message>> {
        if self.busy {
            return Task::none();
        }
        self.confirm_clear = false;
        // Activate the page in the model.
        self.nav.activate(id);
        let users = self.users.clone();
        for user in users {
            if self.nav.text(id).is_some_and(|f| f == user.to_string()) {
                self.selected_user = Some(user);
            }
        }

        self.update_title()
    }
}

impl AppModel {
    /// The about page for this app.
    pub fn about(&self) -> Element<'_, Message> {
        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

        let icon = widget::svg(widget::svg::Handle::from_memory(APP_ICON));

        let title = text::title3(fl!("app-title"));

        let hash = env!("VERGEN_GIT_SHA");
        let short_hash: String = hash.chars().take(7).collect();
        let date = env!("VERGEN_GIT_COMMIT_DATE");

        let link = widget::button::link(REPOSITORY)
            .on_press(Message::OpenRepositoryUrl)
            .padding(0);

        widget::column()
            .push(icon)
            .push(title)
            .push(link)
            .push(
                widget::button::link(fl!(
                    "git-description",
                    hash = short_hash.as_str(),
                    date = date
                ))
                .on_press(Message::LaunchUrl(format!("{REPOSITORY}/commits/{hash}")))
                .padding(0),
            )
            .align_x(Alignment::Center)
            .spacing(space_xxs)
            .into()
    }

    fn list_fingers_task(&self) -> Task<cosmic::Action<Message>> {
        if let (Some(proxy), Some(user)) = (&self.device_proxy, &self.selected_user) {
            let proxy = proxy.clone();
            let username = (*user.username).clone();
            return Task::perform(
                async move {
                    match list_enrolled_fingers_dbus(&proxy, username).await {
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

    fn on_cancel_clear(&mut self) -> Task<cosmic::Action<Message>> {
        self.confirm_clear = false;
        Task::none()
    }

    fn on_clear_completion(&mut self, res: Result<(), AppError>) -> Task<cosmic::Action<Message>> {
        match res {
            Ok(_) => {
                self.status = fl!("device-cleared");
                self.enrolled_fingers.clear();
            }
            Err(e) => {
                self.status = e.localized_message();
            }
        }
        self.busy = false;
        Task::none()
    }

    fn on_clicked_link(&mut self) -> Task<cosmic::Action<Message>> {
        let _ = open::that_detached(REPOSITORY);
        Task::none()
    }

    fn on_connection_ready(&mut self, conn: zbus::Connection) -> Task<cosmic::Action<Message>> {
        self.connection = Some(conn.clone());
        self.status = fl!("status-searching-device");

        let conn_clone = conn.clone();
        let find_device_task = Task::perform(
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
        );

        let conn_clone = conn.clone();
        // Get users from AccountsService
        let fetch_users_task = Task::perform(
            async move {
                let mut users = Vec::new();
                if let Ok(accounts) = AccountsProxy::new(&conn_clone).await
                    && let Ok(user_paths) = accounts.list_cached_users().await
                {
                    let fetched_users: Vec<_> = stream::iter(user_paths)
                        .map(|path| {
                            let conn = conn_clone.clone();
                            async move {
                                let builder = match UserProxy::builder(&conn).path(&path) {
                                    Ok(builder) => builder,
                                    Err(e) => {
                                        tracing::error!(
                                            %e,
                                            "Failed to create UserProxy for path {path}"
                                        );
                                        return Err(e);
                                    }
                                };

                                if let Ok(user_proxy) = builder.build().await {
                                    if let (Ok(name), Ok(real_name), Ok(icon)) = (
                                        user_proxy.user_name().await,
                                        user_proxy.real_name().await,
                                        user_proxy.icon().await,
                                    ) {
                                        Ok::<_, zbus::Error>(UserOption {
                                            username: Arc::new(name),
                                            realname: Arc::new(real_name),
                                            icon: Arc::new(icon),
                                        })
                                    } else {
                                        Err(zbus::Error::Failure(
                                            "Failed to fetch user name or real name".to_string(),
                                        ))
                                    }
                                } else {
                                    Err(zbus::Error::Failure(
                                        "Failed to fetch user name or real name".to_string(),
                                    ))
                                }
                            }
                        })
                        .buffered(USER_FETCH_CONCURRENCY)
                        .filter_map(|res| async { res.ok() })
                        .collect()
                        .await;
                    users.extend(fetched_users);
                }

                // Fallback to current user if list is empty
                if users.is_empty()
                    && let Ok(Some(user)) = User::from_uid(Uid::current())
                {
                    users.push(UserOption {
                        username: Arc::new(user.name),
                        realname: Arc::new(user.gecos.to_string_lossy().into_owned()),
                        icon: Arc::new("".to_string()),
                    });
                }
                Message::UsersFound(users)
            },
            cosmic::Action::App,
        );

        Task::batch(vec![find_device_task, fetch_users_task])
    }

    fn on_context_page_toggle(
        &mut self,
        context_page: ContextPage,
    ) -> Task<cosmic::Action<Message>> {
        if self.context_page == context_page {
            // Close the context drawer if the toggled context page is the same.
            self.core.window.show_context = !self.core.window.show_context;
        } else {
            // Open the context drawer to display the requested context page.
            self.context_page = context_page;
            self.core.window.show_context = true;
        }
        Task::none()
    }

    fn on_error(&mut self, err: AppError) -> Task<cosmic::Action<Message>> {
        self.status = err.localized_message();
        self.busy = false;
        self.enrolling_finger = None;
        Task::none()
    }

    fn on_fingers_listed(&mut self, fingers: Vec<String>) -> Task<cosmic::Action<Message>> {
        self.enrolled_fingers = fingers;
        Task::none()
    }

    fn on_users_found(&mut self, users: Vec<UserOption>) -> Task<cosmic::Action<Message>> {
        self.nav = nav_bar::Model::default();
        for user in &users {
            let id = self.nav.insert().text(user.to_string()).id();
            if self
                .selected_user
                .as_ref()
                .map_or(false, |u| u.username == user.username)
            {
                self.nav.activate(id);
            }
        }

        self.users = users;

        // Ensure selected_user is valid and update it if necessary.
        if let Some(selected) = &self.selected_user {
            if !self.users.iter().any(|u| u.username == selected.username) {
                if !self.users.is_empty() {
                    self.selected_user = Some(self.users[0].clone());
                }
            } else if let Some(updated_user) =
                self.users.iter().find(|u| u.username == selected.username)
            {
                // Update selected_user with the fresh data from the fetched list.
                self.selected_user = Some(updated_user.clone());
            }
        } else if !self.users.is_empty() {
            self.selected_user = Some(self.users[0].clone());
        }

        self.list_fingers_task()
    }

    fn on_finger_selected(&mut self, finger: String) -> Task<cosmic::Action<Message>> {
        if self.busy {
            return Task::none();
        }
        self.confirm_clear = false;
        for fingers in Finger::all() {
            if fingers.localized_name() == finger {
                self.selected_finger = *fingers;
                break;
            }
        }
        Task::none()
    }

    fn on_device_found(
        &mut self,
        device_info: Option<(zbus::zvariant::OwnedObjectPath, DeviceProxy<'static>)>,
    ) -> Task<cosmic::Action<Message>> {
        if let Some((path, proxy)) = device_info {
            self.device_path = Some(Arc::new(path));
            self.device_proxy = Some(proxy);
            self.status = fl!("status-device-found");
            self.busy = false;

            if self.selected_user.is_some() {
                self.list_fingers_task()
            } else {
                Task::none()
            }
        } else {
            self.device_path = None;
            self.device_proxy = None;
            self.status = fl!("status-no-device-found");
            self.busy = true;
            Task::none()
        }
    }

    fn on_enroll_start(&mut self, total: Option<u32>) -> Task<cosmic::Action<Message>> {
        self.enroll_total_stages = total;
        self.enroll_progress = 0;
        self.status = fl!("enroll-starting");
        Task::none()
    }

    fn on_enroll_status(&mut self, status: String, done: bool) -> Task<cosmic::Action<Message>> {
        let status_msg = match status.as_str() {
            "enroll-stage-passed" => {
                self.enroll_progress += 1;
                fl!("enroll-stage-passed")
            }
            "enroll-retry-scan" => fl!("enroll-retry-scan"),
            "enroll-swipe-too-short" => fl!("enroll-swipe-too-short"),
            "enroll-finger-not-centered" => fl!("enroll-finger-not-centered"),
            "enroll-remove-and-retry" => fl!("enroll-remove-and-retry"),
            "enroll-unknown-error" => fl!("enroll-unknown-error"),
            "enroll-completed" => fl!("enroll-completed"),
            "enroll-failed" => fl!("enroll-failed"),
            "enroll-disconnected" => fl!("enroll-disconnected"),
            "enroll-data-full" => fl!("enroll-data-full"),
            "enroll-too-fast" => fl!("enroll-too-fast"),
            "enroll-duplicate" => fl!("enroll-duplicate"),
            "enroll-cancelled" => fl!("enroll-cancelled"),
            _ => status.clone(),
        };
        self.status = status_msg;

        if done {
            self.busy = false;
            self.enrolling_finger = None;

            if status == "enroll-completed" {
                return self.list_fingers_task();
            }
        }
        Task::none()
    }

    fn on_enroll_stop(&self) -> Task<cosmic::Action<Message>> {
        if let (Some(path), Some(conn)) = (self.device_path.clone(), self.connection.clone()) {
            let path = (*path).clone();
            return Task::perform(
                async move {
                    let device = DeviceProxy::builder(&conn).path(path)?.build().await?;
                    let _ = device.enroll_stop().await;
                    device.release().await?;
                    Ok::<(), zbus::Error>(())
                },
                |res| match res {
                    Ok(_) => cosmic::Action::App(Message::EnrollStatus(
                        "enroll-cancelled".to_string(),
                        true,
                    )),
                    Err(e) => cosmic::Action::App(Message::OperationError(AppError::from(e))),
                },
            );
        }
        Task::none()
    }

    fn on_clear_device(&mut self) -> Task<cosmic::Action<Message>> {
        if !self.confirm_clear {
            self.confirm_clear = true;
            return Task::none();
        }

        if let (Some(path), Some(conn)) = (self.device_path.clone(), self.connection.clone()) {
            self.status = fl!("clearing-device");
            self.busy = true;
            self.confirm_clear = false;
            let path = (*path).clone();
            let usernames: Vec<String> = self.users.iter().map(|u| (*u.username).clone()).collect();
            return Task::perform(
                async move {
                    match clear_all_fingers_dbus(&conn, path, usernames).await {
                        Ok(_) => Message::ClearComplete(Ok(())),
                        Err(e) => Message::ClearComplete(Err(AppError::from(e))),
                    }
                },
                cosmic::Action::App,
            );
        }
        Task::none()
    }

    fn on_delete(&mut self) -> Task<cosmic::Action<Message>> {
        if let (Some(path), Some(conn), Some(user)) = (
            self.device_path.clone(),
            self.connection.clone(),
            self.selected_user.clone(),
        ) {
            self.status = fl!("deleting");
            self.busy = true;
            let path = (*path).clone();
            let username = (*user.username).clone();

            if let Some(finger_name) = self.selected_finger.as_finger_id() {
                let finger_name = finger_name.to_string();
                return Task::perform(
                    async move {
                        match delete_fingerprint_dbus(&conn, path, finger_name, username).await {
                            Ok(_) => Message::DeleteComplete,
                            Err(e) => Message::OperationError(AppError::from(e)),
                        }
                    },
                    cosmic::Action::App,
                );
            } else {
                return Task::perform(
                    async move {
                        match delete_fingers(&conn, path, username).await {
                            Ok(_) => Message::DeleteComplete,
                            Err(e) => Message::OperationError(AppError::from(e)),
                        }
                    },
                    cosmic::Action::App,
                );
            }
        }
        Task::none()
    }

    fn on_delete_complete(&mut self) -> Task<cosmic::Action<Message>> {
        self.status = fl!("deleted");
        self.busy = false;
        if let Some(page) = self.nav.data::<Finger>(self.nav.active()) {
            if let Some(finger_id) = page.as_finger_id() {
                self.enrolled_fingers.retain(|f| f != finger_id);
            } else {
                self.enrolled_fingers.clear();
            }
        }
        Task::none()
    }

    fn on_open_link(&mut self, url: String) -> Task<cosmic::Action<Message>> {
        match open::that_detached(&url) {
            Ok(()) => Task::none(),
            Err(err) => {
                eprintln!("failed to open {url:?}: {err}");
                Task::none()
            }
        }
    }

    fn on_register(&mut self) -> Task<cosmic::Action<Message>> {
        self.busy = true;
        if let Some(finger_id) = self.selected_finger.as_finger_id() {
            self.enrolling_finger = Some(Arc::new(finger_id.to_string()));
        }
        self.status = fl!("status-starting-enrollment");
        Task::none()
    }

    /// Updates the header and window titles.
    pub fn update_title(&mut self) -> Task<cosmic::Action<Message>> {
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

    fn on_update_config(&mut self, config: Config) -> Task<cosmic::Action<Message>> {
        self.config = config;
        Task::none()
    }

    fn view_finger_picker(&self) -> Option<Element<'_, Message>> {
        let mut vec = Vec::new();

        for page in Finger::all() {
            vec.push(page.localized_name())
        }

        Some(
            pick_list(
                vec,
                Some(self.selected_finger.localized_name()),
                Message::FingerSelected,
            )
            .width(Length::Fixed(200.0))
            .apply(widget::container)
            .width(Length::Fill)
            .align_x(Horizontal::Center)
            .into(),
        )
    }

    fn view_icon(&self) -> Element<'_, Message> {
        widget::svg(widget::svg::Handle::from_memory(FPRINT_ICON))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_status(&self) -> Element<'_, Message> {
        widget::text(&self.status)
            .size(STATUS_TEXT_SIZE)
            .apply(widget::container)
            .width(Length::Fill)
            .align_x(Horizontal::Center)
            .into()
    }

    fn view_progress(&self) -> Option<Element<'_, Message>> {
        self.enrolling_finger.as_ref()?;

        self.enroll_total_stages.map(|total| {
            widget::progress_bar(0.0..=(total as f32), self.enroll_progress as f32)
                .height(PROGRESS_BAR_HEIGHT)
                .into()
        })
    }

    fn view_controls(&self) -> Element<'_, Message> {
        let buttons_enabled =
            !self.busy && self.device_path.is_some() && self.enrolling_finger.is_none();

        let current_finger = self.selected_finger.as_finger_id();
        let is_enrolled = if let Some(f) = current_finger {
            self.enrolled_fingers.iter().any(|ef| ef == f)
        } else {
            !self.enrolled_fingers.is_empty()
        };

        let register_btn = widget::button::text(fl!("register"));
        let delete_btn = widget::button::text(fl!("delete"));
        let clear_btn = widget::button::text(fl!("clear-device"));

        let register_btn = if buttons_enabled && current_finger.is_some() {
            register_btn.on_press(Message::Register)
        } else {
            register_btn
        };

        let delete_btn = if buttons_enabled && is_enrolled {
            delete_btn.on_press(Message::Delete)
        } else {
            delete_btn
        };

        let clear_btn =
            if !self.busy && self.device_path.is_some() && self.enrolling_finger.is_none() {
                clear_btn.on_press(Message::ClearDevice)
            } else {
                clear_btn
            };

        let mut cancel_btn = widget::button::text(fl!("cancel"));
        if self.enrolling_finger.is_some() {
            cancel_btn = cancel_btn.on_press(Message::EnrollStop);
        }

        let mut row = widget::row()
            .push(register_btn)
            .push(delete_btn)
            .push(clear_btn);

        if self.enrolling_finger.is_some() {
            row = row.push(cancel_btn);
        }

        row.apply(widget::container)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(MAIN_PADDING)
            .into()
    }
}
