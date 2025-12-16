// 别名
type NanoSecond = u64;
type Inch = u64;

fn main() {
    let decimal = 65.4321_f32;
    // 显示转换类型
    let integer = decimal as u8;
    let character = integer as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    let n: NanoSecond = 5;
    let i: Inch = 2;
    println!("n is {n}, i is {i}");
}
