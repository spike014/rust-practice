fn main() {
    let mut arr = Vec::new();
    arr.push(22); // 增加元素
    println!("{:?}", arr);

    /*
    如果预先知道要存储的元素个数，可以使用 Vec::with_capacity(capacity) 
    创建动态数组，这样可以避免因为插入大量新数据导致频繁的内存分配和拷贝，提升性能
    */

    // 使用宏 vec![] 创建动态数组
    let mut arr1 = vec![33, 1, 44];
    arr1.push(22);
    println!("{:?}", arr1);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("第三个元素是 {}", third);

    // 安全模式获取，防止数组越界访问导致程序错误，意外退出
    match v.get(9) {
        Some(nine) => println!("第 9 个元素是 {}", nine),
        None => println!("f去你的第 9 个元素，根本没有！"),
    }

    // 遍历访问数组
    for i in &v {
        println!({}, i);
    }
    // 使用 for in 访问比下标访问更加高效安全，
    // 因为使用下标访问每次都会触发数组边界检查
}
