mod my {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents }
        }
    }
}

fn main() {
    // 公有结构体可以在任何地方访问
    let open_box = my::OpenBox { contents: "public information" };
    println!("open_box contains: {}", open_box.contents);

    // 可以用公有方法访问私有结构体
    let closed_box = my::ClosedBox::new("classified information");
    // println!("closed_box contains: {}", closed_box.contents); // 这里会报错，因为 closed_box 是私有的
}
