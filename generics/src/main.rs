#![allow(unused)]
fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
    a + b
}

struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct PointDiff<T, U> {
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

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = &item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);



    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p2 = PointDiff {x:111, y: 22.1};
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

// Rust 实现了泛型，使得使用泛型类型参数的代码相比使用具体类型并没有任何速度上的损失。

// Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。
// 单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。

// 编译器寻找所有泛型代码被调用的位置并使用泛型代码针对具体类型生成代码。

// 我们可以使用泛型来编写不重复的代码，而 Rust 将会为每一个实例编译其特定类型的代码。
// 这意味着在使用泛型时没有运行时开销；当代码运行，它的执行效率就跟好像手写每个具体定义的重复代码一样。
// 这个单态化过程正是 Rust 泛型在运行时极其高效的原因。
