pub trait Draw {
    fn draw(&self) -> String; 
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self) 
    } 
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self) 
    } 
}

fn draw1(x: Box<dyn Draw>) {
    println!("1 {}", x.draw());
}

// Box<dyn Draw> 与 &dyn Draw 区别
// 因为 Box<dyn> 是一个宽指针（fat pointer），它需要一次额外的解引用后，才能获取到指向 vtable 的指针，
// 然后再通过该指针访问 vtable 查询到具体的函数指针，最后进行调用。

fn draw2(x: &dyn Draw) {
    println!("2 {}", x.draw());
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn main() {
    let x = 1.1f64;
    // do_something(&x);
    let y = 8u8;

    draw1(Box::new(x));
    draw1(Box::new(y));
    draw2(&x);
    draw2(&y);

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) -> String {
        println!("{}", self.width);
        println!("{}", self.height);
        println!("{}", self.label);
        // 绘制按钮的代码
        format!("Button: {:?}", self) 

    }
}

#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) -> String {
        println!("{}", self.width);
        println!("{}", self.height);
        println!("{:?}", self.options);
        // 绘制SelectBox的代码
        format!("SelectBox: {:?}", *self) 
    }
}
