# lib_dice

> A dice parsing library uses nom for dice syndax parsing

## Usage

Cargo.toml
```toml
[dependencies]
lib_dice = "0.1.0"
```

src/main.rs
```rust
extern crate lib_dice;

fn main() {
	let roll_1 = lib_dice::roll(1, 8, 0); // 1d8 + 0
	let roll_2 = lib_dice::roll_from_str("2d6 + 7");
}
```

## Example

```
$ git clone https://github.com/automas-dev/lib_dice.git
$ cd lib_dice
$ cargo run --example demo
```
The following prompt will start. Close the demo with `CTRL + C`
```
Welcome to Dice Line v0.1.0
> 1d8
3
> 3d6 + 8
16
```

## Licence

lib_dice uses the [MIT](LICENCE) Licence
