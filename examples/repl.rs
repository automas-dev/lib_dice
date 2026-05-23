use std::io;
use std::io::prelude::*;

fn main() {
    println!("Welcome to Dice Line v{}", env!("CARGO_PKG_VERSION"));
    println!("Press ctrl+c to exit");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("STDIN Error");

        match lib_dice::roll_from_str(&line.trim()) {
            Ok(res) => println!("{}", res),
            Err(err) => println!("Error: {}", err),
        }
    }
}
