struct Point {
    x: f64,
    y: f64,
}

impl Point {
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

impl Rectangle {
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
        (width + height) * 2
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

}
