fn main() {
    // 创建一个空String
    let mut s = String::new();
    // 将&str类型的"hello,world"添加到s中
    s.push_str("hello,world");
    // 将字符'!'推入s中
    s.push('!');
    // 最后s的内容是"hello,world!"
    assert_eq!(s,"hello,world!");

    // 从现有的&str切片创建String类型
    let mut s = "hello,world".to_string();
    // 将字符'!'推入s中
    s.push('!');
    // 最后s的内容是"hello,world!"
    assert_eq!(s,"hello,world!");

    // 从现有的&str切片创建String类型
    // String与&str都是UTF8编码，因此支持中文
    let mut s = String::from("你好,世界");
    // 将字符'!'推入s中
    s.push('!');
    // 最后s的内容是"你好,世界!"
    assert_eq!(s,"你好,世界!");

    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    assert_eq!(s3,"hello,world!");
    // 下面的语句如果去掉注释，就会报错
    // println!("{}",s1);
    // 可继续使用 s2
    println!("{}",s2);

    let s1 = String::from("hello");
    // let h = &s1[0]; complie error
    // let h = s1[0]; complie error
    let h = &s1[0..1];
    println!("{}",h);

}
