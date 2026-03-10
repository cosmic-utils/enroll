// SPDX-License-Identifier: MPL-2.0

use crate::app::AppModel;
use crate::app::Finger;
use crate::app::message::Message;
use crate::fl;
use cosmic::iced::Length;
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced_widget::pick_list;
use cosmic::widget::{button, column, container, progress_bar, row, svg, text};
use cosmic::{Apply, Element};
const FPRINT_ICON: &[u8] = include_bytes!("../../resources/icons/hicolor/scalable/apps/fprint.svg");
const STATUS_TEXT_SIZE: u16 = 16;
pub(crate) const MAIN_PADDING: u16 = 20;
pub(crate) const MAIN_SPACING: u16 = 20;

impl AppModel {
    pub(crate) fn view_main(&self) -> Element<'_, Message> {
        let left_hand = row()
            .push(self.finger_button(Finger::LeftPinky, 110.0))
            .push(self.finger_button(Finger::LeftRing, 140.0))
            .push(self.finger_button(Finger::LeftMiddle, 150.0))
            .push(self.finger_button(Finger::LeftIndex, 130.0))
            .push(self.finger_button(Finger::LeftThumb, 40.0))
            .spacing(10)
            .align_y(Vertical::Bottom);

        let right_hand = row()
            .push(self.finger_button(Finger::RightThumb, 40.0))
            .push(self.finger_button(Finger::RightIndex, 130.0))
            .push(self.finger_button(Finger::RightMiddle, 150.0))
            .push(self.finger_button(Finger::RightRing, 140.0))
            .push(self.finger_button(Finger::RightPinky, 110.0))
            .spacing(10)
            .align_y(Vertical::Bottom);

        let mut column = column();

        if self.core.is_condensed() {
            column = column
                .push(
                    container(left_hand)
                        .width(Length::Fill)
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .padding(MAIN_PADDING),
                )
                .push(
                    container(right_hand)
                        .width(Length::Fill)
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .padding(MAIN_PADDING),
                );
        } else {
            let hands = row()
                .push(left_hand)
                .push(right_hand)
                .spacing(50)
                .align_y(Vertical::Bottom);
            column = column.push(
                container(hands)
                    .width(Length::Fill)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .padding(MAIN_PADDING),
            );
        }
        column = column.push(self.view_status());

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

    fn finger_button(&self, finger: Finger, height: f32) -> Element<'_, Message> {
        let is_selected = self.selected_finger == finger;
        let is_enrolled = finger
            .as_finger_id()
            .is_some_and(|id| self.enrolled_fingers.iter().any(|ef| ef == id));
        let mut svg = svg(svg::Handle::from_memory(FPRINT_ICON)).symbolic(true);
        let label = text(finger.localized_name()).size(10);
        if is_enrolled {
            svg = svg.opacity(0.7);
        }
        let col = column().push(svg).push(label);
        let container = container(col);

        button::custom_image_button(container, None)
            .width(40)
            .height(Length::Fixed(height))
            .on_press(Message::FingerSelected(finger.localized_name()))
            .selected(is_selected)
            //.description(label)
            .into()
    }

    pub(crate) fn view_old(&self) -> Element<'_, Message> {
        let mut column = column().push(self.view_header()).push(self.view_status());

        if let Some(picker) = self.view_finger_picker() {
            column = column.push(picker);
        }

        if let Some(progress) = self.view_progress() {
            column = column.push(progress);
        }

        column
            .push(self.view_icon())
            .push(self.view_controls())
            .align_x(Horizontal::Center)
            .spacing(MAIN_SPACING)
            .padding(MAIN_PADDING)
            .into()
    }

    pub(crate) fn view_header(&self) -> Element<'_, Message> {
        text::title1(fl!("app-title"))
            .apply(container)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into()
    }

    pub(crate) fn view_finger_picker(&self) -> Option<Element<'_, Message>> {
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
            .apply(container)
            .width(Length::Fill)
            .align_x(Horizontal::Center)
            .into(),
        )
    }

    pub(crate) fn view_icon(&self) -> Element<'_, Message> {
        svg(svg::Handle::from_memory(FPRINT_ICON))
            .symbolic(true)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub(crate) fn view_status(&self) -> Element<'_, Message> {
        text(&self.status)
            .size(STATUS_TEXT_SIZE)
            .apply(container)
            .width(Length::Fill)
            .align_x(Horizontal::Center)
            .into()
    }

    pub(crate) fn view_progress(&self) -> Option<Element<'_, Message>> {
        self.enrolling_finger.as_ref()?;

        self.enroll_total_stages
            .map(|total| progress_bar(0.0..=(total as f32), self.enroll_progress as f32).into())
    }

    pub(crate) fn view_controls(&self) -> Element<'_, Message> {
        let buttons_enabled =
            !self.busy && self.device_path.is_some() && self.enrolling_finger.is_none();

        let current_finger = self.selected_finger.as_finger_id();
        let is_enrolled = if let Some(f) = current_finger {
            self.enrolled_fingers.iter().any(|ef| ef == f)
        } else {
            !self.enrolled_fingers.is_empty()
        };

        let register_btn = button::text(fl!("register")).tooltip(fl!("register-tooltip"));
        let verify_btn = button::text(fl!("verify")).tooltip(fl!("verify-tooltip"));
        let delete_btn = button::text(fl!("delete")).tooltip(fl!("delete-tooltip"));

        let register_btn = if buttons_enabled && current_finger.is_some() {
            register_btn.on_press(Message::Register)
        } else {
            register_btn
        };

        let verify_btn = if buttons_enabled && current_finger.is_some() {
            verify_btn.on_press(Message::VerifyFinger)
        } else {
            verify_btn
        };

        let delete_btn = if buttons_enabled && is_enrolled {
            delete_btn.on_press(Message::Delete)
        } else {
            delete_btn
        };

        let mut cancel_btn = button::text(fl!("cancel"));
        if self.enrolling_finger.is_some() {
            cancel_btn = cancel_btn.on_press(Message::EnrollStop);
        }

        let mut row = row().push(register_btn).push(verify_btn).push(delete_btn);

        if self.enrolling_finger.is_some() {
            row = row.push(cancel_btn);
        }

        row.apply(container)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(MAIN_PADDING)
            .into()
    }
}
