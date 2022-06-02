use rand::Rng;
use std::io;

fn main() {
    println!("游戏开始！！！");

    let x: [u8; 2] = [1, 2];
    let [x1, x2] = x;
    println!("x: {:?}", x);

    let y: [String; 2] = [String::from("1"), String::from("2")];
    let z: &str = "3";
    let zz: &str = &z[..];
    // let [y1, y2] = y;
    let y1 = y[0];
    let yy = &y[..];
    println!("y: {:?}", y);

    let mut q = [1, 2, 3];
    let qq = &q[..];
    let mut qqq = &mut q[..];
    qqq[1] = 1;
    // println!("q: {:?}", qq);
    // q[0] = 1;
    // println!("q: {:?}", qq);

    // let w = (1, 2, 3);
    let ww = &w(..); // &(i32,i32,32)
    let www = &w(0..1); // &(i32)

    // 生成 4 个随机数字

    let mut random: [u8; 4] = [0, 0, 0, 0];

    for i in 0..random.len() {
        random[i] = {
            let mut n;
            'mark: loop {
                n = rand::thread_rng().gen_range(0, 10);
                for j in 0..i {
                    if random[j] == n {
                        continue 'mark;
                    }
                }
                break;
            }
            n
        };
    }

    let mut times = 0;
    loop {
        times += 1;

        if times > 8 {
            println!("没猜出来，你输了！");
            break;
        }

        let mut guessing = String::new();

        io::stdin().read_line(&mut guessing).expect("读取失败。");

        let guessing = guessing.trim();

        if guessing.len() != 4 {
            println!("请输入 4 位数字");
            times -= 1;
            continue;
        }

        let _guessing: u32 = match guessing.parse() {
            Ok(v) => v,
            Err(_e) => {
                println!("请输入 4 位数字");
                times -= 1;
                continue;
            }
        };

        let mut right = 0;
        let mut position = 0;

        for i in 0..guessing.len() {
            let g: u8 = guessing[i..i+1].parse().expect("解析失败");
            for j in 0..random.len() {
                if random[j] == g {
                    right += 1;
                    if j == i {
                        position += 1;
                    }
                }
            }
        }

        if position == 4 {
            println!("你猜的数字是: {}  你猜对了，真厉害！！！ 只用了 {} 次机会", guessing, times);
            break;
        }

        println!("第{}次: 你猜的数字是: {}   {}个数字正确, {}个数字位置也正确", times, guessing, right, position);
    }
}
