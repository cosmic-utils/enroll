// SPDX-License-Identifier: MPL-2.0
use crate::app::message::{Message, REPOSITORY};
use crate::app::tasks::task_connect;
use crate::app::{ContextPage, MenuAction};
use crate::app::{error::*, finger::*, fprint::*, subscription::*, users::*};
use crate::config::{Config, read_config};
use crate::fl;
use cosmic::app::context_drawer;

use cosmic::iced::{Alignment, Subscription};
use cosmic::{
    cosmic_theme,
    prelude::*,
    theme,
    widget::{self, column, dialog, menu, nav_bar, text},
};

use super::AppModel;
use std::collections::HashMap;

const APP_ICON: &[u8] = include_bytes!("../../resources/icons/hicolor/scalable/apps/enroll.svg");

/// Turns AppModel to a COSMIC application
impl cosmic::Application for AppModel {
    /// The async executor that will be used to run your application's commands.
    type Executor = cosmic::executor::Default;

    /// Data that your application receives to its init method.
    type Flags = ();

    /// Messages which the application and its widgets will emit.
    type Message = Message;

    /// Unique identifier in RDNN (reverse domain name notation) format.
    const APP_ID: &'static str = "org.cosmic_utils.enroll";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    /// Initializes the application with any given flags and startup commands.
    fn init(
        mut core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        // Gets users
        let (users, nav, selected_user) = initialize_users();

        // Load configuration
        let (config_handler, config) = read_config(Self::APP_ID);

        // Start with navigation closed
        core.nav_bar_toggle();

        let mut app = AppModel {
            core,
            context_page: ContextPage::About,
            nav,
            key_binds: HashMap::new(),
            config,
            config_handler,
            status: fl!("status-connecting"),
            device_path: None,
            device_proxy: None,
            connection: None,
            busy: true,
            enrolling_finger: None,
            verifying_finger: false,
            enroll_progress: 0,
            enroll_total_stages: None,
            users,
            selected_user,
            selected_finger: Finger::default(),
            enrolled_fingers: Vec::new(),
            confirm_clear: false,
        };

        let start_theme = cosmic::command::set_theme(app.config.app_theme.theme());
        let command = app.update_title_task();
        let connect_task = task_connect();

        (app, Task::batch(vec![command, connect_task, start_theme]))
    }

    /// Elements to pack at the start of the header bar.
    fn header_start(&self) -> Vec<Element<'_, Self::Message>> {
        let menu_bar = menu::bar(vec![menu::Tree::with_children(
            Element::from(menu::root(fl!("view"))),
            menu::items(
                &self.key_binds,
                vec![
                    menu::Item::Button(fl!("about"), None, MenuAction::About),
                    menu::Item::Button(fl!("settings"), None, MenuAction::Settings),
                ],
            ),
        )]);

        vec![menu_bar.into()]
    }

    /// Enables the COSMIC application to create a nav bar with this model.
    fn nav_model(&self) -> Option<&nav_bar::Model> {
        if self.nav.len() > 1 {
            Some(&self.nav)
        } else {
            None
        }
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
            ContextPage::Settings => context_drawer::context_drawer(
                self.settings(),
                Message::ToggleContextPage(ContextPage::Settings),
            )
            .title(fl!("settings")),
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

    /// Chooses which view to render based on config
    fn view(&self) -> Element<'_, Self::Message> {
        if self.config.experimental_ui {
            self.view_old()
        } else {
            self.view_main()
        }
    }

    /// Register subscriptions for this application.
    ///
    /// Subscriptions are long-running async tasks running in the background which
    /// emit messages to the application through a channel. They are started at the
    /// beginning of the application, and persist through its lifetime.
    fn subscription(&self) -> Subscription<Self::Message> {
        struct MySubscription;

        let mut subscriptions = vec![
            // Create a subscription which emits updates through a channel.
            Subscription::run_with(std::any::TypeId::of::<MySubscription>(), |_id| {
                cosmic::iced::stream::channel(4, move |_channel| async move {
                    futures_util::future::pending().await
                })
            }),
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
            let data = EnrollData::new(
                finger_name.clone(),
                device_path.clone(),
                connection.clone(),
                user.username.clone(),
            );

            subscriptions.push(enroll_subscription(data));
        }

        // Add verify subscription if verifying
        if self.verifying_finger
            && let (Some(device_path), Some(connection), Some(user)) =
                (&self.device_path, &self.connection, &self.selected_user)
        {
            let data = VerifyData::new(
                device_path.clone(),
                connection.clone(),
                user.username.clone(),
                self.selected_finger
                    .as_finger_id()
                    .unwrap_or_default()
                    .to_string(),
            );

            subscriptions.push(verify_subscription(data));
        }

        subscriptions.push(portal_theme_subscription(self.config.app_theme));

        Subscription::batch(subscriptions)
    }

    /// Handles messages emitted by the application and its widgets.
    ///
    /// Tasks may be returned for asynchronous execution of code in the background
    /// on the application's async runtime.
    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::ConnectionReady(conn) => self.on_connection_ready(conn),
            Message::FingerSelected(finger) => self.on_finger_selected(finger),
            Message::DeviceFound(path) => self.on_device_found(path),
            Message::EnrolledFingers(fingers) => self.on_fingers_listed(fingers),
            Message::OperationError(err) => self.on_error(err),
            Message::EnrollStart(total) => self.on_enroll_start(total),
            Message::EnrollStatus(status, done) => self.on_enroll_status(status, done),
            Message::EnrollStop => self.on_enroll_stop(),
            Message::DeleteComplete(clear) => self.on_delete_complete(clear),
            Message::Delete => self.on_delete(),
            Message::ClearDevice => self.on_clear_device(),
            Message::CancelClear => self.on_cancel_clear(),
            Message::ClearComplete(res) => self.on_clear_completion(res),
            Message::Register => self.on_register(),
            Message::OpenRepositoryUrl => self.on_clicked_link(),
            Message::ToggleContextPage(context_page) => self.on_context_page_toggle(context_page),
            Message::UpdateConfig(config) => self.on_update_config(config),
            Message::LaunchUrl(url) => self.on_open_link(url),
            Message::VerifyFinger => self.on_verify_finger(),
            Message::VerifyStatus(status, done) => self.on_verify_status(status, done),
            Message::ThemeChanged(is_dark) => self.on_portal_color_scheme_changed(is_dark),
            Message::ThemeSetting(theme) => self.on_theme_setting(theme),
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

        Task::batch(vec![self.update_title_task(), self.list_fingers_task()])
    }
}

// TODO: about & settings could be in view. others in tasks.
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

        column()
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
    fn update_title_task(&mut self) -> Task<cosmic::Action<Message>> {
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
