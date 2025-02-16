use std::vec;

use teloxide::types::{
    KeyboardButton,
    KeyboardMarkup
};

pub const ENTROPY_BUTTON: &str = "Entropy";
pub const HELP_BUTTON: &str = "Help";
pub const INFO_BUTTON: &str = "Info";
pub const CANCEL_BUTTON: &str = "Cancel";

pub fn menu_keyboard() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![
        vec![KeyboardButton::new(ENTROPY_BUTTON)],
        vec![KeyboardButton::new(HELP_BUTTON)],
        vec![KeyboardButton::new(INFO_BUTTON)]
    ]).persistent()
}

pub fn cancel_keyboard() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![
        vec![KeyboardButton::new(CANCEL_BUTTON)]
    ])
}