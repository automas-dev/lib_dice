# lib_dice

A dice parsing library uses nom for dice syntax parsing.

- [Documentation](https://docs.rs/lib_dice/latest/lib_dice)
- [Crates IO](https://crates.io/crates/lib_dice)

Dice string format is `{count}d{sides}` or `{count}d{sides}+{bonus}` with to
include a bonus. The regex form is `\d+d\d+(\s*\+\s*\d+)?` or with capture
groups `(\d+)d(\d+)(?:\s*\+\s*(\d+))?`

## Usage

Cargo.toml
```toml
[dependencies]
lib_dice = "1.3.0"
```

src/main.rs

```rust
use lib_dice::DiceRoll;
use std::str::FromStr;

fn main() {
	let roll_1 = lib_dice::roll(1, 8, 0); // 1d8 + 0
	let roll_2 = lib_dice::roll_from_str("2d6 + 7").unwrap();

    let dice = DiceRoll::new(1, 8, 3); // 1d8 + 3
    let roll_3 = dice.roll();
    let dice = DiceRoll::from_str("1d12").unwrap();
    let roll_4 = dice.roll_no_bonus();
}
```

## Example

```sh
git clone https://github.com/automas-dev/lib_dice.git
cd lib_dice
cargo run --example repl
```

The following prompt will start.

```
Welcome to Dice Line v1.1.0
Press ctrl+c to exit
> 1d8
3
> 3d6 + 8
16
```

## License

lib_dice uses the [MIT](LICENSE) License
