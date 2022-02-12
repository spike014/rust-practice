use std::ops::Add;

#[derive(Debug)]
struct Point<T: Add<T, Output = T>> { // T must implement Add trait
    x: T,
    y: T,
}

// implement Add for T
impl <T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, p: Point<T>) -> Point<T> {
        Point{
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

fn add<T: Add<T, Output=T>>(a:T, b:T) -> T {
    a + b
}

fn main() {
    let p1 = Point{x: 1.1f32, y: 1.2f32};
    let p2 = Point{x: 2.2f32, y: 3.4f32};
    println!("{:?}", add(p1, p2));

    let p1 = Point{x: 1i32, y: 3i32};
    let p2 = Point{x: 2i32, y: 4i32};
    println!("{:?}", add(p1, p2));
}