fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
    a + b
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 针对特定泛型类型进行方法定义，只有该泛型类型才可以进行实现
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    println!("add i8: {}", add(2i8, 3i8));
    println!("add i32: {}", add(20, 30));
    println!("add f64: {}", add(1.23, 1.23));

    // let p = Point{x: 1, y :1.1}; 类型不匹配
}

// <const N: usize> 基于值的泛型
