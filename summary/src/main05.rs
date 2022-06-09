enum MyEnum {
    Foo,
    Bar,
}

fn main() {
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];

    // v.iter().filter(|x| x == MyEnum::Foo);

    if let MyEnum::Foo = v[0] {
        println!("123")
    }

    println!("{}", i8::MAX);

    let v1 = 51_u8 + 8;
    let v2 = i8::checked_add(51, 8).unwrap();
    println!("{},{}", v1, v2);
}
