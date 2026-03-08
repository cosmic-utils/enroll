// SPDX-License-Identifier: MPL-2.0
use crate::app::error::*;
use crate::app::finger::*;
use crate::app::fprint::*;
use crate::app::message::{Message, REPOSITORY};
use crate::app::users::*;
use crate::app::{ContextPage, MenuAction};
use crate::config::Config;
use crate::fl;

use cosmic::app::context_drawer;
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::iced::{Alignment, Subscription};
use cosmic::prelude::*;
use cosmic::widget::{self, column, dialog, menu, nav_bar, settings::view_column, text};
use cosmic::{cosmic_theme, theme};

use super::AppModel;
use futures_util::SinkExt;
use std::collections::HashMap;

const APP_ICON: &[u8] = include_bytes!("../../resources/icons/hicolor/scalable/apps/enroll.svg");

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
        let (users, nav, selected_user) = initialize_users();
        // Construct the app model with the runtime's core.
        let mut app = AppModel {
            core,
            context_page: ContextPage::About,
            nav,
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
            users,
            selected_user,
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
            self.view_experimental()
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

        Task::batch(vec![self.update_title(), self.list_fingers_task()])
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

    /// Settings menu
    pub fn settings(&self) -> Element<'_, Message> {
        let col = column().push(
            widget::checkbox(fl!("alternative-ui"), self.config.experimental_ui).on_toggle(
                |value| {
                    Message::UpdateConfig(Config {
                        experimental_ui: value,
                    })
                },
            ),
        );
        view_column(vec![col.into()]).into()
    }

    pub(crate) fn list_fingers_task(&self) -> Task<cosmic::Action<Message>> {
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
}
