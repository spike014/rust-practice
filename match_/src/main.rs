fn main() {
    // 无论是是 match 还是 if let，
    // 他们都可以在模式匹配时覆盖掉老的值，绑定新的值:
    // 注意，只有在 match 内会覆盖旧的值。
    let age = Some(30);
    println!("在匹配前，age是{:?}", age);
    if let Some(age) = age {
        println!("匹配出来的age是{}", age);
    }
    println!("在匹配后，age是{:?}", age);
    // =======================================
    let age = Some(31);
    println!("在匹配前，age是{:?}", age);
    match age {
        Some(age) => println!("匹配出来的age是{}", age),
        _ => (),
    }
    println!("在匹配后，age是{:?}", age);

    match 1 {
        num @ (1 | 2) => {
            println!("{}", num);
        }
        _ => {}
    }
}

// more about match in
// https://course.rs/basic/match-pattern/all-patterns.html
