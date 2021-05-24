use std::io::{self};
use anyhow::*;

enum Class {
    Mage,
    Warrior,
    Hunter,
}

fn handle_chosen_class(char_str: &str) -> Result<Class, Error> {
    match char_str.to_ascii_lowercase().as_str() {
        "mage\n" => Ok(Class::Mage),
        "warrior\n" => Ok(Class::Warrior),
        "hunter\n" => Ok(Class::Hunter),
        _ => Err(anyhow!("Please select a valid class."))
    }
}

fn main() -> () {
    let mut buf = String::new();

    println!("Choose your character:");
    io::stdin()
        .read_line(&mut buf)
        .expect("Couldn't read input");

    // TODO: handle this better
    handle_chosen_class(&buf).expect("Couldn't select a class.");
}
