struct Point {
    x: f64,
    y: f64,
}

impl Point { // 两个都是静态方法，不需要实例化就可以调用，通过 :: 调用
    // 原点
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    
    // 新点
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

// 矩形
struct Rectangle {
    p1: Point, // 左上角
    p2: Point, // 右下角
}

impl Rectangle { // 都是实例方法，需要实例化才能调用，通过 . 调用
    // 面积
    fn area(&self) -> f64 {
        let width = self.p2.x - self.p1.x;
        let height = self.p2.y - self.p1.y;
        width * height
    }

    // 周长
    fn perimeter(&self) -> f64 {
        let width = self.p2.x - self.p1.x;
        let height = self.p2.y - self.p1.y;
        (width + height) * 2.0
    }

    // 平移
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p1.y += y;
        self.p2.x += x;
        self.p2.y += y;
    }
}

fn main() {
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };
    println!("Rectangle area is {}", rectangle.area());
    println!("Rectangle perimeter is {}", rectangle.perimeter());
    // rectangle.translate(1.0, 1.0); // 这里会报错，因为 rectangle 是不可变的
    println!("Rectangle is at ({}, {})", rectangle.p1.x, rectangle.p1.y);
    println!("Rectangle is at ({}, {})", rectangle.p2.x, rectangle.p2.y);

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };
    square.translate(1.8, 1.6); // 可变结构体可以调用可变方法
    println!("Square is at ({}, {})", square.p1.x, square.p1.y);
    println!("Square is at ({}, {})", square.p2.x, square.p2.y);
}
