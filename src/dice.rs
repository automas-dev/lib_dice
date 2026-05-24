use std::fmt;
use std::str::FromStr;

use nom::IResult;

use crate::error::DiceFormatError;
use crate::parse::parse_roll;

/// A single dice roll configuration including count, dice and bonus
#[derive(Debug, PartialEq)]
pub struct DiceRoll {
    pub count: usize,
    pub sides: u8,
    pub bonus: i32,
}

impl DiceRoll {
    /// Create a new DiceRoll from parameters
    ///
    /// # Example
    ///
    /// ```
    /// use lib_dice::DiceRoll;
    /// let d = DiceRoll::new(1, 6, 3);
    /// assert_eq!(d.count, 1);
    /// assert_eq!(d.sides, 6);
    /// assert_eq!(d.bonus, 3);
    /// ```
    pub fn new(count: usize, sides: u8, bonus: i32) -> Self {
        DiceRoll {
            count,
            sides,
            bonus,
        }
    }

    /// Simulate a random dice roll using the rand crate.
    ///
    /// ## Example
    ///
    /// ```
    /// use lib_dice::DiceRoll;
    /// let dice = DiceRoll::new(1, 8, 0); // 1d8
    /// let roll_1 = dice.roll();
    /// let roll_2 = dice.roll();
    /// ```
    pub fn roll(&self) -> i32 {
        self.roll_no_bonus() + self.bonus
    }

    /// Simulate a random dice roll using the rand crate.
    ///
    /// This method excludes the bonus
    ///
    /// ## Example
    ///
    /// ```
    /// use lib_dice::DiceRoll;
    /// let dice = DiceRoll::new(1, 8, 3); // 1d8+3
    /// let roll_1 = dice.roll_no_bonus();
    /// let roll_2 = dice.roll_no_bonus();
    /// ```
    pub fn roll_no_bonus(&self) -> i32 {
        let max = self.sides as usize * self.count;
        rand::random_range(self.count..=max) as i32
    }
}

impl fmt::Display for DiceRoll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.bonus != 0 {
            write!(f, "{}d{}+{}", self.count, self.sides, self.bonus)
        } else {
            write!(f, "{}d{}", self.count, self.sides)
        }
    }
}

impl FromStr for DiceRoll {
    type Err = DiceFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_roll(s) {
            IResult::Ok((_, (count, sides, bonus))) => Ok(DiceRoll::new(count, sides, bonus)),
            _ => Err(DiceFormatError::new(s)),
        }
    }
}

#[test]
fn test_dice_roll() {
    // DiceRoll::new
    assert_eq!(
        DiceRoll {
            count: 1,
            sides: 8,
            bonus: 3
        },
        DiceRoll::new(1, 8, 3)
    );

    // Test range of values
    let dice = DiceRoll::new(1, 6, 0);
    // Roll one dice
    for _ in 0..1000 {
        let r = dice.roll();
        assert!(r >= 1);
        assert!(r <= 6);
    }

    // Roll two dice
    let dice = DiceRoll::new(2, 6, 0);
    for _ in 0..1000 {
        let r = dice.roll();
        assert!(r >= 1);
        assert!(r <= 12);
    }

    // Roll with bonus
    let dice = DiceRoll::new(1, 6, 6);
    for _ in 0..1000 {
        let r = dice.roll();
        assert!(r >= 6);
        assert!(r <= 12);
    }
}

#[test]
fn test_dice_roll_from_str() {
    // Single die
    assert_eq!(
        Ok(DiceRoll {
            count: 1,
            sides: 8,
            bonus: 0
        }),
        DiceRoll::from_str("1d8")
    );

    // Multiple dice
    assert_eq!(
        Ok(DiceRoll {
            count: 3,
            sides: 12,
            bonus: 0
        }),
        DiceRoll::from_str("3d12")
    );

    // Bonus
    assert_eq!(
        Ok(DiceRoll {
            count: 1,
            sides: 8,
            bonus: 3
        }),
        DiceRoll::from_str("1d8+3")
    );

    // Bonus with spaces
    assert_eq!(
        Ok(DiceRoll {
            count: 1,
            sides: 8,
            bonus: 3
        }),
        DiceRoll::from_str("1d8 + 3")
    );

    // Invalid format
    for s in ["", "1", "d", "1d", "d3"] {
        assert_eq!(Err(DiceFormatError::new(s)), DiceRoll::from_str(s));
    }
}
