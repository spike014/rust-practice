#![allow(unused)]

fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
    a + b
}

struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}
// 泛型参数不宜过多，应该多考虑拆分结构体和减少代码复杂度

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

    let p2 = Point2 {x:111, y: 22.1};
    println!("{:?}", p2);

    println!("add i8: {}", add(2i8, 3i8));
    println!("add i32: {}", add(20, 30));
    println!("add f64: {}", add(1.23, 1.23));

    // let p = Point{x: 1, y :1.1}; 类型不匹配

    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr: [i32; 2] = [1, 2];
    display_array(arr);
}

// <const N: usize> 基于值的泛型
fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}
