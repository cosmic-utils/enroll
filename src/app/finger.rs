// SPDX-License-Identifier: MPL-2.0

use crate::fl;

/// The page to display in the application.
///
/// Or buttons depending default UI.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
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
    DeleteAllUsersPrints,
}

impl Finger {
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
            Self::DeleteAllUsersPrints,
        ]
    }

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
            Self::DeleteAllUsersPrints => fl!("page-delete-all-users-prints"),
        }
    }

    /// Maps keys 1-0 to fingers (1=RightThumb, 0=LeftPinky).
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

    pub fn as_finger_id(&self) -> Option<&'static str> {
        match self {
            Finger::RightThumb => Some("right-thumb"),
            Finger::RightIndex => Some("right-index-finger"),
            Finger::RightMiddle => Some("right-middle-finger"),
            Finger::RightRing => Some("right-ring-finger"),
            Finger::RightPinky => Some("right-little-finger"),
            Finger::LeftThumb => Some("left-thumb"),
            Finger::LeftIndex => Some("left-index-finger"),
            Finger::LeftMiddle => Some("left-middle-finger"),
            Finger::LeftRing => Some("left-ring-finger"),
            Finger::LeftPinky => Some("left-little-finger"),
            Finger::DeleteAllUsersPrints => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_all() {
        let pages = Finger::all();
        assert_eq!(pages.len(), 11);
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
        assert_eq!(pages[10], Finger::DeleteAllUsersPrints);
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
        assert!(!Finger::DeleteAllUsersPrints.localized_name().is_empty());
    }

    #[test]
    fn test_finger_from_key() {
        assert_eq!(Finger::from_key(6), Some(Finger::RightThumb));
        assert_eq!(Finger::from_key(7), Some(Finger::RightIndex));
        assert_eq!(Finger::from_key(8), Some(Finger::RightMiddle));
        assert_eq!(Finger::from_key(9), Some(Finger::RightRing));
        assert_eq!(Finger::from_key(0), Some(Finger::RightPinky));
        assert_eq!(Finger::from_key(1), Some(Finger::LeftThumb));
        assert_eq!(Finger::from_key(2), Some(Finger::LeftIndex));
        assert_eq!(Finger::from_key(3), Some(Finger::LeftMiddle));
        assert_eq!(Finger::from_key(4), Some(Finger::LeftRing));
        assert_eq!(Finger::from_key(5), Some(Finger::LeftPinky));
        assert_eq!(Finger::from_key(10), None);
    }

    #[test]
    fn test_page_as_finger_id() {
        assert_eq!(Finger::RightThumb.as_finger_id(), Some("right-thumb"));
        assert_eq!(
            Finger::RightIndex.as_finger_id(),
            Some("right-index-finger")
        );
        assert_eq!(
            Finger::RightMiddle.as_finger_id(),
            Some("right-middle-finger")
        );
        assert_eq!(Finger::RightRing.as_finger_id(), Some("right-ring-finger"));
        assert_eq!(
            Finger::RightPinky.as_finger_id(),
            Some("right-little-finger")
        );
        assert_eq!(Finger::LeftThumb.as_finger_id(), Some("left-thumb"));
        assert_eq!(Finger::LeftIndex.as_finger_id(), Some("left-index-finger"));
        assert_eq!(
            Finger::LeftMiddle.as_finger_id(),
            Some("left-middle-finger")
        );
        assert_eq!(Finger::LeftRing.as_finger_id(), Some("left-ring-finger"));
        assert_eq!(Finger::LeftPinky.as_finger_id(), Some("left-little-finger"));
        assert_eq!(Finger::DeleteAllUsersPrints.as_finger_id(), None);
    }
}
