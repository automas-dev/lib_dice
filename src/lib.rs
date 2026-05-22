//! A library for parcing dice rolls using nom.
//!
//! ## Usage
//! ```
//! extern crate lib_dice;
//! use lib_dice::DiceRoll;
//! use std::str::FromStr;
//!
//! fn main() {
//! 	let roll_1 = lib_dice::roll(1, 8, 0); // 1d8 + 0
//! 	let roll_2 = lib_dice::roll_from_str("2d6 + 7");
//!
//!     let dice = DiceRoll::new(1, 8, 3); // 1d8 + 3
//!     let roll_3 = dice.roll();
//!     let roll_4 = dice.roll_no_bonus();
//! }
//! ```

extern crate nom;
extern crate rand;

use std::str::FromStr;

mod error;
pub use error::DiceFormatError;

mod dice;
pub use dice::DiceRoll;

mod parse;

/// Simulate a random dice roll using the rand crate.
///
/// ## Example
/// ```
/// use lib_dice::roll;
/// let roll_1 = roll(1, 8, 0); // 1d8 + 0
/// let roll_2 = roll(2, 6, 7); // 2d6 + 7
/// ```
pub fn roll(count: usize, dice: u8, bonus: i32) -> i32 {
    DiceRoll { count, dice, bonus }.roll()
}

/// Parse a dice roll from a sting in one of the following formats
///
/// ## Example
/// ```
/// use lib_dice::roll_from_str;
/// let roll_1 = roll_from_str("1d8").unwrap();  // roll a single 8 sided dice
/// let roll_2 = roll_from_str("2d6 + 7").unwrap();  // roll 2 6 sided dice with a +7 bonus
/// ```
///
/// ## Error
///
/// A `lib_dice::DiceFormatError` will be thrown if the dice string is malformed or empty.
pub fn roll_from_str(cmd: &str) -> Result<i32, DiceFormatError> {
    Ok(DiceRoll::from_str(cmd)?.roll())
}

#[test]
fn test_roll() {
    // Roll one dice
    for _ in 0..1000 {
        let r = roll(1, 6, 0);
        assert!(r >= 1);
        assert!(r <= 6);
    }

    // Roll two dice
    for _ in 0..1000 {
        let r = roll(2, 6, 0);
        assert!(r >= 1);
        assert!(r <= 12);
    }

    // Roll with bonus
    for _ in 0..1000 {
        let r = roll(1, 6, 6);
        assert!(r >= 6);
        assert!(r <= 12);
    }
}

#[test]
fn test_roll_from_str() {
    // Roll one dice
    for _ in 0..1000 {
        let r = roll_from_str("1d6").unwrap();
        assert!(r >= 1);
        assert!(r <= 6);
    }

    // Roll two dice
    for _ in 0..1000 {
        let r = roll_from_str("2d6").unwrap();
        assert!(r >= 1);
        assert!(r <= 12);
    }

    // Roll with bonus
    for _ in 0..1000 {
        let r = roll_from_str("1d6+6").unwrap();
        assert!(r >= 6);
        assert!(r <= 12);
    }
}
