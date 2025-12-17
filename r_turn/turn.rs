// 只要为类型 U 实现了 From<T>，那么 T 会自动获得 Into<U> 的实现（无需手动写）。

// // 为 String 实现 From<&str>（标准库已内置）
// impl From<&str> for String {
//     fn from(s: &str) -> Self {
//         s.to_string()
//     }
// }

fn main() {
    // into
    // 自动获得 &str 的 Into<String> 实现
    let s: &str = "hello";
    let s_string: String = s.into(); // 无需手动实现 Into
    println!("My string is {}", s_string);

    // from
    let s_string2 = String::from("hello");
    println!("My string is {}", s_string2);
}
