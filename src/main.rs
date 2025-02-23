use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Uhodni cislo!");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Zadejte cele cislo, prosim.");
        println!();

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Nepodarilo se precist cislo.");

        if guess.trim() == "quit" {
            println!("Zamyslene cislo: {}", secret_number);
            break;
        }

        let number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match number.cmp(&secret_number) {
            Ordering::Less => println!("Prelis male"),
            Ordering::Greater => println!("Prelis velke"),
            Ordering::Equal => {
                println!("Uhodli jste!");
                break;
            }
        }
    }
}
