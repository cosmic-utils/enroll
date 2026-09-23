use crate::app::{AppModel, message::Message};
use crate::fl;
use cosmic::widget::segmented_control;
use cosmic::widget::settings::item::builder;
use cosmic::widget::settings::{section, view_column};
use cosmic::{
    Element, cosmic_theme, theme,
    widget::{Column, button, radio, text},
};

impl AppModel {
    /// Settings menu
    pub fn settings(&self) -> Element<'_, Message> {
        let cosmic_theme::Spacing { space_xs, .. } = theme::active().cosmic().spacing;

        let theme_selection = builder(fl!("settings-theme")).flex_control(
            segmented_control::horizontal(&self.theme)
                .style(theme::SegmentedButton::Control)
                .on_activate(Message::ThemeSetting),
        );

        let theme_section = section()
            .title(fl!("settings-ui"))
            .add(theme_selection)
            .add(builder(fl!("alternative-ui")).toggler(false, Message::UpdateUI));

        let device_count = self.devices.len();

        let mut device_section = section().title(fl!("settings-section-devices"));

        for (index, device) in self.devices.iter().enumerate() {
            let is_selected = self
                .device_path
                .as_ref()
                .is_some_and(|p| **p == device.path);

            device_section = device_section.add(
                builder(fl!("settings-supported"))
                    .description(fl!("settings-device", nbr = device_count))
                    .control(radio(
                        text::heading(&device.name),
                        index,
                        if is_selected { Some(index) } else { None },
                        Message::SelectDevice,
                    )),
            );
        }

        let clear_btn = button::destructive(fl!("clear-device")).tooltip(fl!("clear-tooltip"));

        let clear_btn =
            if !self.busy && self.device_path.is_some() && self.enrolling_finger.is_none() {
                clear_btn.on_press(Message::ClearDevice)
            } else {
                clear_btn
            };

        let clear_section = section()
            .title(fl!("danger"))
            .add(builder(fl!("settings-clear-device")).control(clear_btn));

        let col = Column::new()
            .push(theme_section)
            .push(device_section)
            .push(clear_section)
            .spacing(space_xs);
        view_column(vec![col.into()]).into()
    }
}
