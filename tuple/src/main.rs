fn main() {
    let tup2: (f32, i32, f64) = (3.2, 300, 11.33332);
    // 可以由 Rust 推导出类型
    let tup = (3.2, 300, 11.222);
    // 使用 . 来根据索引访问
    println!("{}", tup.2);

    // 类似 js 的模式匹配解构取值
    let (_, _y, z) = tup2;
    println!("{}", z);

    let str = String::from("hello");
    let (s, len) = calculate_length(str);
    println!("String `{}` length is {}.", s, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)

    // 直接
    // (s, s.len())
    /*
        error[E0382]: borrow of moved value: `s`
      --> src/main.rs:18:9
       |
    17 | fn calculate_length(s: String) -> (String, usize) {
       |                     - move occurs because `s` has type `String`, which does not implement the `Copy` trait
    18 |     (s, s.len())
       |      -  ^^^^^^^ value borrowed here after move
       |      |
       |      value moved here
    */
}
