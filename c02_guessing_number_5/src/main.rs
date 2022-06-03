use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Game started!");

    let random = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("input number");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("read failed");

        let guess: u32 = match guess.trim().parse() {
            Ok(v) => v,
            Err(_e) => {
                println!("not a number");
                continue;
            }
        };

        match guess.cmp(&random) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("win");
                break;
            }
        }
    }
}
