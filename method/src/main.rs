pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    pub fn width(&self) -> u32 {
        return self.width;
    }

    /*
    需要注意的是，self 依然有所有权的概念：
        self 表示 Rectangle 的所有权转移到该方法中，这种形式用的较少
        &self 表示该方法对 Rectangle 的不可变借用
        &mut self 表示可变借用
    */
}

fn main() {
    let rect1 = Rectangle::new(30, 50);

    println!("width: {}", rect1.width());
    println!("height: {}", rect1.height);
}
