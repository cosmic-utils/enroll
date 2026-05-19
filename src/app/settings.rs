use crate::app::{AppModel, message::Message};
use crate::config::{AppTheme, Config};
use crate::fl;
use cosmic::widget::settings::item::builder;
use cosmic::widget::settings::{item_row, section, view_column};
use cosmic::{
    Element, cosmic_theme, theme,
    widget::{Column, button, checkbox, radio, text},
};

impl AppModel {
    /// Settings menu
    pub fn settings(&self) -> Element<'_, Message> {
        let cosmic_theme::Spacing { space_xs, .. } = theme::active().cosmic().spacing;
        let clear_btn = button::text(fl!("clear-device")).tooltip(fl!("clear-tooltip"));

        let clear_btn =
            if !self.busy && self.device_path.is_some() && self.enrolling_finger.is_none() {
                clear_btn.on_press(Message::ClearDevice)
            } else {
                clear_btn
            };

        let theme_section = section()
            .title(fl!("settings-ui"))
            .add(
                builder(fl!("settings-theme"))
                    .control(item_row(vec![
                        radio(
                            text::heading(fl!("theme-system")),
                            AppTheme::System,
                            Some(self.config.app_theme),
                            Message::ThemeSetting,
                        )
                        .into(),
                        radio(
                            text::heading(fl!("theme-light")),
                            AppTheme::Light,
                            Some(self.config.app_theme),
                            Message::ThemeSetting,
                        )
                        .into(),
                        radio(
                            text::heading(fl!("theme-dark")),
                            AppTheme::Dark,
                            Some(self.config.app_theme),
                            Message::ThemeSetting,
                        )
                        .into(),
                    ]))
                    .wrap(),
            )
            .add(
                builder(fl!("alternative-ui")).control(
                    checkbox(self.config.experimental_ui)
                        .on_toggle(|value| {
                            Message::UpdateConfig(Config {
                                app_theme: self.config.app_theme,
                                experimental_ui: value,
                            })
                        })
                        .label(fl!("alternative-ui")),
                ),
            );

        let device_count = self.devices.iter().count();

        let mut device_section = section().title(fl!("settings-device", nbr = device_count));

        for (index, device) in self.devices.iter().enumerate() {
            let is_selected = self
                .device_path
                .as_ref()
                .is_some_and(|p| **p == device.path);

            device_section = device_section.add(radio(
                text::heading(&device.name),
                index,
                if is_selected { Some(index) } else { None },
                Message::SelectDevice,
            ));
        }

        let clear_section = section()
            .title(fl!("danger"))
            .add(builder(fl!("settings-clear-device")).control(item_row(vec![clear_btn.into()])));

        let col = Column::new()
            .push(theme_section)
            .push(device_section)
            .push(clear_section)
            .spacing(space_xs);
        view_column(vec![col.into()]).into()
    }
}
