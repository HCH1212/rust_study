fn main() {
    // 闭包
    let closure = |i| i + 1;
    println!("closure is {}", closure(1));

    // 闭包捕获
    let base = 10;
    let add_base = |i| i + base;
    println!("add_base is {}", add_base(1));

    // 1. 借 fn闭包：不修改捕获的变量，原变量还能继续用
    let name = String::from("Rust");
    let say_hello = || println!("Hello, {}", name);
    println!("原变量：{}", name);
    say_hello();

    // 2. 拿 fnmut闭包：修改捕获的变量，原变量暂时不能用
    let mut count = 0;
    let mut add_one = || {
        count += 1;
        println!("count: {}", count);
    };
    add_one();
    add_one();
    println!("最终count: {}", count);

    // 3. 偷 fnonce闭包：把变量的所有权拿过来，原作用域不能用了
    let s = String::from("我是要被拿走的字符串");
    let take_s = move || {
        println!("闭包里的s：{}", s);
    };
    // println!("原变量s：{}", s);
    take_s();
}
