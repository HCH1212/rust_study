fn main() {
    let boolean = true;

    let binary = match boolean {
        // match 分支必须覆盖所有可能的值
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);


    // 解构元组
    let triple = (0, -2, 3);
    println!("Tell me about {:?}", triple);

    match triple {
        // 规则1：匹配“第一个元素是 0”的三元组，同时把第二个元素绑定到变量 y，第三个绑定到 z
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        
        // 规则2：匹配“第一个元素是 1”的三元组，`..` 表示“剩下的元素不管是什么都忽略”
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        
        // 规则3：`_` 是“通配符”，匹配所有没被上面规则命中的情况（类似 switch 的 default）
        _ => println!("It doesn't matter what they are"),
    }

    // 卫语句
    let pair = (2, -2);    
    println!("Tell me about {:?}", pair);

    match pair {
        (x, y) if x == y => println!("These are twins"),
        // ^ `if` 条件部分是一个卫语句
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}
