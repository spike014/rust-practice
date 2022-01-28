fn main() {
    let number = if 1 + 1 == 3 { 5 } else { 6 };

    println!("The value of number is: {}", number);

    for i in 0..=10 {
        print!(" {}", i);
    }

    println!("");
    let a = [4, 3, 2, 1];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }
    println!("{:?}", a);
}
