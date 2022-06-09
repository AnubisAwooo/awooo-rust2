use std::collections::HashMap;

struct Counter {
    max: u32,
    current: u32,
}

impl Counter {
    fn _current(&self) -> u32 {
        self.current
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.max {
            None
        } else {
            self.current += 1;
            Some(self.current - 1)
        }
    }
}

trait Iterator2 {
    type Item1;
    type Item2;

    fn next(self) -> Option<(Self::Item1, Self::Item2)>;
}

impl Iterator2 for Counter {
    type Item1 = u32;
    type Item2 = String;
    fn next(self) -> Option<(Self::Item1, Self::Item2)> {
        Some((1, "123".to_string()))
    }
}

fn main() {
    println!("Hello, world!");

    let _map: HashMap<String, String> = HashMap::new();

    // let _v = &map["123"];

    for i in (Counter {
        current: 5,
        max: 10,
    }) {
        println!("{}", i);
    }

    let c = Counter {
        current: 1,
        max: 10,
    };
    let _o = Iterator2::next(c);
}
