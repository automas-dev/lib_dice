use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct DiceFormatError {
    dice_string: String,
}

impl DiceFormatError {
    pub fn new(dice_string: &str) -> Self {
        DiceFormatError {
            dice_string: String::from(dice_string),
        }
    }

    pub fn get_dice_string(&self) -> &str {
        &self.dice_string
    }
}

impl Display for DiceFormatError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Dice string was formatted incorrectly '{}'",
            self.get_dice_string()
        )
    }
}

impl Error for DiceFormatError {}
