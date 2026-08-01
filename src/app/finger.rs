// SPDX-License-Identifier: MPL-2.0

use crate::fl;

/// The page to display in the application.
///
/// Or buttons depending default UI.
#[derive(Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Finger {
    RightThumb,
    #[default]
    RightIndex,
    RightMiddle,
    RightRing,
    RightPinky,
    LeftThumb,
    LeftIndex,
    LeftMiddle,
    LeftRing,
    LeftPinky,
}

impl Finger {
    /// Static reference to a finger struct instance
    pub fn all() -> &'static [Self] {
        &[
            Self::RightThumb,
            Self::RightIndex,
            Self::RightMiddle,
            Self::RightRing,
            Self::RightPinky,
            Self::LeftThumb,
            Self::LeftIndex,
            Self::LeftMiddle,
            Self::LeftRing,
            Self::LeftPinky,
        ]
    }

    /// Maps a name to Fluent localized string
    pub fn localized_name(&self) -> String {
        match self {
            Self::RightThumb => fl!("page-right-thumb"),
            Self::RightIndex => fl!("page-right-index-finger"),
            Self::RightMiddle => fl!("page-right-middle-finger"),
            Self::RightRing => fl!("page-right-ring-finger"),
            Self::RightPinky => fl!("page-right-little-finger"),
            Self::LeftThumb => fl!("page-left-thumb"),
            Self::LeftIndex => fl!("page-left-index-finger"),
            Self::LeftMiddle => fl!("page-left-middle-finger"),
            Self::LeftRing => fl!("page-left-ring-finger"),
            Self::LeftPinky => fl!("page-left-little-finger"),
        }
    }

    /// Maps keys 1-0 to fingers (1=LeftPinky, 0=RightPinky).
    pub fn from_key(key: u8) -> Option<Finger> {
        match key {
            1 => Some(Finger::LeftPinky),
            2 => Some(Finger::LeftRing),
            3 => Some(Finger::LeftMiddle),
            4 => Some(Finger::LeftIndex),
            5 => Some(Finger::LeftThumb),
            6 => Some(Finger::RightThumb),
            7 => Some(Finger::RightIndex),
            8 => Some(Finger::RightMiddle),
            9 => Some(Finger::RightRing),
            0 => Some(Finger::RightPinky),
            _ => None,
        }
    }

    /// Maps to fprintd API name
    pub fn as_finger_id(&self) -> &'static str {
        match self {
            Finger::RightThumb => "right-thumb",
            Finger::RightIndex => "right-index-finger",
            Finger::RightMiddle => "right-middle-finger",
            Finger::RightRing => "right-ring-finger",
            Finger::RightPinky => "right-little-finger",
            Finger::LeftThumb => "left-thumb",
            Finger::LeftIndex => "left-index-finger",
            Finger::LeftMiddle => "left-middle-finger",
            Finger::LeftRing => "left-ring-finger",
            Finger::LeftPinky => "left-little-finger",
        }
    }
}

impl std::fmt::Display for Finger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.localized_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_all() {
        let pages = Finger::all();
        assert_eq!(pages.len(), 10);
        assert_eq!(pages[0], Finger::RightThumb);
        assert_eq!(pages[1], Finger::RightIndex);
        assert_eq!(pages[2], Finger::RightMiddle);
        assert_eq!(pages[3], Finger::RightRing);
        assert_eq!(pages[4], Finger::RightPinky);
        assert_eq!(pages[5], Finger::LeftThumb);
        assert_eq!(pages[6], Finger::LeftIndex);
        assert_eq!(pages[7], Finger::LeftMiddle);
        assert_eq!(pages[8], Finger::LeftRing);
        assert_eq!(pages[9], Finger::LeftPinky);
    }

    #[test]
    fn test_page_localized_name() {
        // Check that localized names are not empty.
        // Note: Actual values depend on the loaded translation, which defaults to fallback (English).
        assert!(!Finger::RightThumb.localized_name().is_empty());
        assert!(!Finger::RightIndex.localized_name().is_empty());
        assert!(!Finger::RightMiddle.localized_name().is_empty());
        assert!(!Finger::RightRing.localized_name().is_empty());
        assert!(!Finger::RightPinky.localized_name().is_empty());
        assert!(!Finger::LeftThumb.localized_name().is_empty());
        assert!(!Finger::LeftIndex.localized_name().is_empty());
        assert!(!Finger::LeftMiddle.localized_name().is_empty());
        assert!(!Finger::LeftRing.localized_name().is_empty());
        assert!(!Finger::LeftPinky.localized_name().is_empty());
    }

    #[test]
    fn test_finger_from_key() {
        assert_eq!(Finger::from_key(1), Some(Finger::LeftPinky));
        assert_eq!(Finger::from_key(2), Some(Finger::LeftRing));
        assert_eq!(Finger::from_key(3), Some(Finger::LeftMiddle));
        assert_eq!(Finger::from_key(4), Some(Finger::LeftIndex));
        assert_eq!(Finger::from_key(5), Some(Finger::LeftThumb));
        assert_eq!(Finger::from_key(6), Some(Finger::RightThumb));
        assert_eq!(Finger::from_key(7), Some(Finger::RightIndex));
        assert_eq!(Finger::from_key(8), Some(Finger::RightMiddle));
        assert_eq!(Finger::from_key(9), Some(Finger::RightRing));
        assert_eq!(Finger::from_key(0), Some(Finger::RightPinky));
        assert_eq!(Finger::from_key(10), None);
    }

    #[test]
    fn test_page_as_finger_id() {
        assert_eq!(Finger::RightThumb.as_finger_id(), "right-thumb");
        assert_eq!(Finger::RightIndex.as_finger_id(), "right-index-finger");
        assert_eq!(Finger::RightMiddle.as_finger_id(), "right-middle-finger");
        assert_eq!(Finger::RightRing.as_finger_id(), "right-ring-finger");
        assert_eq!(Finger::RightPinky.as_finger_id(), "right-little-finger");
        assert_eq!(Finger::LeftThumb.as_finger_id(), "left-thumb");
        assert_eq!(Finger::LeftIndex.as_finger_id(), "left-index-finger");
        assert_eq!(Finger::LeftMiddle.as_finger_id(), "left-middle-finger");
        assert_eq!(Finger::LeftRing.as_finger_id(), "left-ring-finger");
        assert_eq!(Finger::LeftPinky.as_finger_id(), "left-little-finger");
    }
}
