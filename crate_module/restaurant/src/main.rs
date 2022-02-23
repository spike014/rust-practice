
mod front_of_house; // 从同名文件中加载 front_of_house 模块

pub use crate::front_of_house::hosting; // 引用模块 front_of_house 的相关内容
// pub front_of_house::hosting; 相对路径引用
// 使用方式: add_to_waitlist();

use rand::Rng;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

fn main() {
    eat_at_restaurant();
    println!("{}", rand::thread_rng().gen_range(1..10))
}
