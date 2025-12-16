// 结构体
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 枚举
enum WebEvent {
    PageLoad,
    PageUnload,
    Click { x: i64, y: i64 },
    KeyPress(char),
    Paste(String),
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        WebEvent::Click { x, y } => println!("Clicked at ({}, {})", x, y),
        WebEvent::KeyPress(c) => println!("Key pressed: {}", c),
        WebEvent::Paste(s) => println!("Pasted: {}", s),
    }
}

fn main() {
    let person = Person {
        name: String::from("John"),
        age: 20,
    };
    println!("person is {:?}", person);
    println!("person name is {}", person.name);
    println!("person age is {}", person.age);

    let event = WebEvent::Click { x: 10, y: 20 };
    inspect(event);
}
