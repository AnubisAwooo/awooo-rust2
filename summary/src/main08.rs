//  修改一行让代码正常打印
fn main() {
    let c1 = "中";
    print_char(&c1.chars().collect::<Vec<char>>()[0]);
}

fn print_char(c: &char) {
    println!("{}", c);
}
