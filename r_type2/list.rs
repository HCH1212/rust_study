// 枚举实现链表
use List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    // 创建一个空列表
    fn new() -> List {
        Nil
    }

    // 添加一个元素到列表
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // 返回列表的长度
    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }
    
    // 返回列表的字符串表示
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify()) // 递归调用 stringify
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    println!("list is {}", list.stringify());
    println!("list length is {}", list.len());
}
