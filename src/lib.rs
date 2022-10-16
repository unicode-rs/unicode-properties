#[rustfmt::skip]
mod tables;

#[cfg(feature = "emoji")]
mod emoji {
    pub use crate::tables::emoji::EmojiStatus;

    pub trait UnicodeEmoji: Sized {
        fn emoji_status(self) -> EmojiStatus;

        fn is_emoji_char(self) -> bool {
            crate::tables::emoji::is_emoji_status_for_emoji_char(self.emoji_status())
        }

        fn is_emoji_component(self) -> bool {
            crate::tables::emoji::is_emoji_status_for_emoji_component(self.emoji_status())
        }
    }

    impl UnicodeEmoji for char {
        fn emoji_status(self) -> EmojiStatus {
            crate::tables::emoji::emoji_status(self)
        }
    }
}

#[cfg(feature = "general-category")]
mod general_category {
    pub use crate::tables::general_category::{GeneralCategory, GeneralCategoryGroup};

    pub trait UnicodeGeneralCategory: Sized {
        fn general_category(self) -> GeneralCategory;

        fn general_category_group(self) -> GeneralCategoryGroup {
            crate::tables::general_category::general_category_group(self.general_category())
        }

        fn is_letter_cased(self) -> bool {
            crate::tables::general_category::general_category_is_letter_cased(
                self.general_category(),
            )
        }
    }

    impl UnicodeGeneralCategory for char {
        fn general_category(self) -> GeneralCategory {
            crate::tables::general_category::general_category_of_char(self)
        }
    }
}

pub use tables::UNICODE_VERSION;

#[cfg(feature = "emoji")]
pub use emoji::UnicodeEmoji;

#[cfg(feature = "emoji")]
pub use emoji::EmojiStatus;

#[cfg(feature = "general-category")]
pub use general_category::GeneralCategory;

#[cfg(feature = "general-category")]
pub use general_category::GeneralCategoryGroup;

#[cfg(feature = "general-category")]
pub use general_category::UnicodeGeneralCategory;
