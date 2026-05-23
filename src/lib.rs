//! A library for parcing dice rolls using nom.
//!
//! ## Usage
//! ```
//! use lib_dice::DiceRoll;
//! use std::str::FromStr;
//!
//! fn main() {
//! 	let roll_1 = lib_dice::roll(1, 8, 0); // 1d8 + 0
//! 	let roll_2 = lib_dice::roll_from_str("2d6 + 7").unwrap();
//!
//!     let dice = DiceRoll::new(1, 8, 3); // 1d8 + 3
//!     let roll_3 = dice.roll();
//!     let dice = DiceRoll::from_str("1d12").unwrap();
//!     let roll_4 = dice.roll_no_bonus();
//! }
//! ```

mod error;
pub use error::DiceFormatError;

mod dice;
pub use dice::DiceRoll;

mod roll;
pub use roll::{roll, roll_from_str};

mod parse;
