use rand::Rng;
use std::io;

fn main() {
    println!("游戏开始！！！");

    struct Color(u8, u8, u8);

    struct User {
        name: String,
        age: u8,
    }

    let user = User {
        name: String::from("John"),
        age: 8,
    };
    let uu = User {
        age: 8,
        ..user
    };

    // user.name = String::from("John2");

    let mut user2 = User {
        name: String::from("John"),
        age: 8,
    };
    user2.name = String::from("John2");

    impl User {
        fn get_name(&self) -> &String {
            &self.name
        }
        fn set_name(&mut self, name: String) {
            self.name = name;
        }
        fn new() -> User {
            User {
                name: String::from("John"),
                age: 8,
            }
        }
    }
    let user3 = User::new();
    user3.get_name();
    // let { name } = user3;
    struct User2 {
        n1: u8,
        n2: u8,
    }
    let u2 = User2 {
        n1: 0,
        n2: 0,
    };
    // let { n1, n2} = u2;
    struct User3(u8, u8);
    let u3 = User3(0,0);
    // let (n1, n2) = u3;
    let [n1, n2] = [1, 2];

    let mut user4 = User::new();
    user4.set_name(String::from("John123"));

    let user5 = &mut user4;
    user5.set_name(String::from("John123"));

    #[derive(Debug)]
    enum Person {
        Student,
        Teacher(f64),
        Teacher2(String),
        T {
            name: String
        },
    }

    let person: Person = Person::Student;
    let (salary) = person;
    println!("{:?}", salary);
    let (salary) = Person::Student;
    println!("{:?}", salary);
    let person: Person = Person::Teacher(1000f64);
    let (salary) = person;
    println!("{:#?}", salary);
    let person: Person = Person::T{
        name: String::from("John123"),
    };
    match &person {
        Student => println!("{:?}", person),
        _ => {},
    }
    if let Person::Teacher(salary) = person {

    }
    // let { name } = person;
    println!("{:?}", person);


    // 生成 4 个随机数字

    let mut random: [u8; 4] = [0, 0, 0, 0];

    for i in 0..random.len() {
        random[i] = random_number(&random[..i]);
    }

    let mut times = 0;
    loop {
        times += 1;

        if times > 8 {
            println!("没猜出来，你输了！");
            break;
        }

        let guessing = match read_guess() {
            Some(v) => v,
            None => {
                times -= 1;
                continue;
            },
        };

        let (right, position) = check(&guessing, &random);

        if position == 4 {
            println!("你猜的数字是: {}  你猜对了，真厉害！！！ 只用了 {} 次机会", guessing, times);
            break;
        }

        println!("第{}次: 你猜的数字是: {}   {}个数字正确, {}个数字位置也正确", times, guessing, right, position);
    }
}

fn random_number(random: &[u8]) -> u8 {
    let mut n;
    'mark: loop {
        n = rand::thread_rng().gen_range(0, 10);
        for j in random {
            if j == &n {
                continue 'mark;
            }
        }
        break;
    }
    n
}

fn read_guess() -> Option<String> {
    let mut guessing = String::new();

    io::stdin().read_line(&mut guessing).expect("读取失败。");

    let guessing = guessing.trim();

    if guessing.len() != 4 {
        println!("请输入 4 位数字");
        return None;
    }

    let _guessing: u32 = match guessing.parse() {
        Ok(v) => return Some(v),
        Err(_e) => {
            println!("请输入 4 位数字");
            return None;
        }
    };
}

fn check(guessing: &String, random: &[u8; 4]) -> (u8, u8) {
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

    (right, position)
}