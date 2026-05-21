#[derive(Debug)]
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

use std::fmt;
impl fmt::Display for DiceFormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bad Format: '{}'", self.get_dice_string())
    }
}

use std::error;
impl error::Error for DiceFormatError {
    fn description(&self) -> &str {
        "Dice string was formatted incorrectly"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
