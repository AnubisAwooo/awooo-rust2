use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Game started !"); 

    let random = rand::thread_rng().gen_range(1, 100);

    loop {
        println!("input a number");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("read failed");

        let guess: u8 = match guess.trim().parse() {
            Ok(v) => v,
            Err(_e) => {
                println!("not a number");
                continue;
            }
        };

        match guess.cmp(&random) {
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("big"),
            Ordering::Equal => {
                println!("win");
                break;
            }
        }

    }

}
