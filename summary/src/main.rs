use std::fmt::{self, Formatter};

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    // 实现 fmt 方法
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "The point is ({}, {})", self.x, self.y)
    }
}

fn main() {
    let origin = Point { x: 0, y: 0 };
    // 填空
    assert_eq!(origin.to_string(), "The point is (0, 0)");
    assert_eq!(
        format!("The point is ({}, {})", origin.x, origin.y),
        "The point is (0, 0)"
    );

    println!("Success!")
}
