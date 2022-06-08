use rand::Rng;
use regex::Regex;
use std::io;

fn main() {
    println!("游戏开始！！！");

    // 生成 4 个随机数字

    let mut random: [u8; 4] = [0, 0, 0, 0];

    for i in 0..random.len() {
        random[i] = random_number(&random[..i]);
    }

    for times in 1..=8 {
        let (guess, guessing) = loop {
            match read_guess() {
                Some(v) => break v,
                None => continue,
            };
        };

        let (right, position) = check(&guess, &random);

        if position == 4 {
            println!(
                "你猜的数字是: {}  你猜对了，真厉害！！！ 只用了 {} 次机会",
                guessing, times
            );
            return;
        }

        println!(
            "第{}次: 你猜的数字是: {}   {}个数字正确, {}个数字位置也正确",
            times, guessing, right, position
        );
    }

    println!("没猜出来，你输了！");
}

fn random_number(random: &[u8]) -> u8 {
    let mut n;
    'mark: loop {
        n = rand::thread_rng().gen_range(0..=10);
        for j in random {
            if *j == n {
                continue 'mark;
            }
        }
        break;
    }
    n
}

fn read_guess() -> Option<([u8; 4], String)> {
    let mut guessing = String::new();

    io::stdin().read_line(&mut guessing).expect("读取失败。");

    let guessing = guessing.trim();

    let re: Regex = Regex::new(r"^\d{4}$").unwrap();

    if !re.is_match(guessing) {
        println!("请输入 4 位数字");
        return None;
    }

    let backup = guessing.clone().to_string();

    let guessing: Vec<u8> = guessing.chars().map(|c| c as u8 - b'0').collect();
    let mut guess: [u8; 4] = [0; 4];

    for i in 0..4 {
        guess[i] = guessing[i];
    }

    Some((guess, backup))
}

fn check(guessing: &[u8; 4], random: &[u8; 4]) -> (u8, u8) {
    let mut right = 0;
    let mut position = 0;

    for i in 0..guessing.len() {
        let g: u8 = guessing[i];
        for j in 0..random.len() {
            if random[j] == g {
                right += 1;
                if j == i {
                    position += 1;
                }
            }
        }
    }

    (right, position)
}
