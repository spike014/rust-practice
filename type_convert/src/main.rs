fn main() {
    // 使用关键字 as
    let a = 3.1 as i8;
    let b = 100_i8 as i32;
    let c = 'a' as u8; // 将字符'a'转换为整数，97
 
    println!("{},{},{}",a,b,c);

    // 使用 try_into 进行安全转换
    let b: i16 = 1500;
    let b_: u8 = match b.try_into() {
        Ok(b1) => b1,
        Err(e) => { // 捕获溢出错误
            println!("转换错误：{:?}", e.to_string());
            0
        }
    };
    println!("b_: {}", b_);

    // https://course.rs/basic/converse.html#%E9%80%9A%E7%94%A8%E7%B1%BB%E5%9E%8B%E8%BD%AC%E6%8D%A2
}
