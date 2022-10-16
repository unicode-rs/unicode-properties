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

pub use tables::UNICODE_VERSION;

#[cfg(feature = "emoji")]
pub use emoji::UnicodeEmoji;

#[cfg(feature = "emoji")]
pub use emoji::EmojiStatus;
