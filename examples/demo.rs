use std::str::FromStr;

use lib_dice::DiceRoll;

fn main() {
    let roll_1 = lib_dice::roll(1, 8, 0); // 1d8 + 0
    let roll_2 = lib_dice::roll_from_str("2d6 + 7").unwrap();

    println!("Roll 1 is {} roll 2 is {}", roll_1, roll_2);

    let dice = DiceRoll::new(1, 8, 3); // 1d8 + 3
    let roll_3 = dice.roll();
    let roll_4 = dice.roll_no_bonus();

    println!(
        "Dice {} rolled 2 times and got [{}, {}]",
        dice, roll_3, roll_4
    );

    let s = "3d12+5";
    let dice = DiceRoll::from_str(s).unwrap();
    print!("Rolling {} got", dice);
    for _ in 0..5 {
        let roll = dice.roll();
        print!(" {}", roll);
    }
    println!("");
}
