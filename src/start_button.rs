use carapax::types::KeyboardButton;
use carapax::types::ReplyKeyboardMarkup;

#[derive(Copy, Clone, /*Display, FromStr*/)]
pub enum UserLocation {
    Location,
}

impl UserLocation {
    pub fn markup() -> ReplyKeyboardMarkup {
        ReplyKeyboardMarkup::default().row(vec![
            KeyboardButton::new("Location"),
        ])
    }
}
