use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {

    println!("Game start......");

    let random = rand::thread_rng().gen_range(1, 101);


    let mut guess ;

    loop {
        guess = String::from("");

        io::stdin().read_line(&mut guess).expect("读取失败");

        let guess: u32 = match guess.trim().parse() {
            Ok(v) => v,
            Err(e) => {
                println!("请输入有效的数字: {}", e);
                continue;
            }
        };

        match guess.cmp(&random) {
            Ordering::Equal => {
                println!("猜中了");
                break;
            },
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了")
        }
    }

}
