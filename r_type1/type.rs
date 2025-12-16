fn main() {
    let mut x = 5;
    x = 6;
    println!("x is {x}");
    
    let x = true; // 可以shadow
    println!("x is {x}");

    // 元组
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; // 解构
    println!("x is {x}, y is {y}, z is {z}");
    println!("x is {0}, y is {1}, z is {2}", tup.0, tup.1, tup.2);

    // 数组
    let arr = [1, 2, 3, 4, 5];
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr = [3; 5]; // [3, 3, 3, 3, 3]
    println!("arr is {:?}", arr);

    // 切片
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
    println!("slice is {:?}", slice);
}
