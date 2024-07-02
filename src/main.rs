use std::process::exit;

fn main() {
    if cfg!(windows) {
        println!(
            "Escoba version {} - Copyright (c) 2024 Gátomo\n Useful broom to clean your RAM in Window$\n\n Made with hatred towards Micro$oft by Gátomo\n https://github.com/gatomod/escoba\n",
            option_env!("CARGO_PKG_VERSION").unwrap_or("😿")
        );

        println!("Allocating escoba...");

        let mut escoba = Vec::<u128>::new();

        println!("Sweeping...");

        loop {
            escoba.push(7251);
        }
    } else {
        println!("You don't need this escoba to clean your RAM because you're not in Window$ (that means that your OS has a decent memory allocator)");
        exit(1)
    }
}
