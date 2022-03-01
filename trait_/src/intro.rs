#![allow(unused)]

use std::process::Output;
pub trait Summary {
    // fn summarize(&self) -> String;
    fn summarize(&self) -> String {     // 默认实现
        String::from("Read more...")
    }
    // 默认实现允许调用相同特征中的其他方法，哪怕这些方法没有默认实现。
    fn show_something(&self) -> String;
    fn show_more(&self) -> String {
        self.show_something();
        "show all".to_string()
    }
}

pub struct Post {
    pub title: String,   // 标题
    pub author: String,  // 作者
    pub content: String, // 内容
}

// 没有默认实现的方法，必须要进行实现
impl Summary for Post {
    fn show_something(&self) -> String{
        "show something".to_string()
    }
} // 此时 Post 调用时，会使用默认实现

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String { // 重写了默认实现
        format!("{}发表了 xxx{}", self.username, self.content)
    }
    fn show_something(&self) -> String{
        "show something".to_string()
    }
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = &item;
        }
    }

    &largest
}

// trait 作为参数
fn show_post(item: impl Summary) {
    println!("{}", item.summarize())
}

// trait bound
// ```
// fn show_post(item: impl Summary) {}
// ==
// fn show_post<T: Summary>(item: T) {}
//
// fn notify(item: impl Summary, item2: impl Summary) {}
// ==
// fn notify<T: Summary>(item: T, item2: T) {} 
//
//
// // 使用 + 进行多个 trait bound
// fn show_post(item: impl Summary + ShowComment) {}
// ==
// fn show_post<T: Summary + ShowComment>(item: T) {}
//
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
// ==
// fn some_function<T, U>(t: T, u: U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {
//
// // 返回值 trait bound
// fn show_post() -> impl Summary {}
//
// // 有条件地实现方法
// impl<T: Display + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {}
// }
// ```
fn main() {
    let post = Post {
        title: "Rust语言简介".to_string(),
        author: "xxxx".to_string(),
        content: "Awesome Rust! Like PHP... (-.-)".to_string(),
    };
    let weibo = Weibo {
        username: "xxx".to_string(),
        content: "?????".to_string(),
    };

    println!("{}", post.summarize());
    println!("{}", weibo.summarize());

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    show_post(weibo);
    show_post(post);
}
