
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("游戏开始!!!");

    let random = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("读取出错");

        let guess: u32 = match guess.trim().parse() {
            Ok(v) => v,
            Err(e) => {
                println!("解析错误 {}", e);
                continue;
            }
        };

        match guess.cmp(&random) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("猜对了");
                break;
            },
        }

    }

}
