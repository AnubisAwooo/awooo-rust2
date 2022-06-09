struct Interface<'a> {
    _manager: &'a mut Manager<'a>,
}

impl<'a> Interface<'a> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}
struct Manager<'a> {
    _text: &'a str,
}

struct List<'a> {
    manager: Manager<'a>,
}

impl<'a> List<'a> {
    pub fn get_interface(&'a mut self) -> Interface {
        Interface {
            _manager: &mut self.manager,
        }
    }
}

fn main() {
    let mut list = List {
        manager: Manager { _text: "hello" },
    };

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    // 下面的调用会失败，因为同时有不可变/可变借用
    // 但是Interface在之前调用完成后就应该被释放了
    // use_list(&list);
}

fn _use_list(list: &List) {
    println!("{}", list.manager._text);
}
