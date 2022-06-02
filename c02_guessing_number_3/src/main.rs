use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Game start...");

    let random = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guessing number.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("read number failed.");

        let guess: u32 = match guess.trim().parse() {
            Ok(v) => v,
            Err(_e) => {
                println!("Please input valid number.");
                continue;
            }
        };

        match guess.cmp(&random) {
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too large."),
            Ordering::Equal => {
                println!("You win!!");
                break;
            },
        }

    }

}
