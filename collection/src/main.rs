use std::collections::HashMap;

fn main() {
    let mut my_maps = HashMap::new();

    my_maps.insert(String::from("new"), 1);
    my_maps.insert(String::from("old"), 2333);

    println!("{:?}", my_maps);

    /*
    跟 Vec 一样，如果预先知道要存储的 KV 对个数，
    可以使用 HashMap::with_capacity(capacity) 创建指定大小的 HashMap，
    避免频繁的内存分配和拷贝，提升性能
    */

    let score: Option<&i32> = my_maps.get(&String::from("new"));
    println!("{:?}", score);
    match score {
        None => {
            println!("Nothing inside.");
        }
        Some(i) => {
            println!("{}", i);
        }
    }

    // 循环取出
    for (key, value) in &my_maps {
        println!("{}: {}", key, value);
    }

    // insert to update
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);

    // 覆盖已有的值
    let old = scores.insert("Blue", 20);
    assert_eq!(old, Some(10));

    // 查询新插入的值
    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(5);
    assert_eq!(*v, 5); // 不存在，插入5

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(50);
    assert_eq!(*v, 5); // 已经存在，因此50没有插入

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    // 根据空格来切分字符串(英文单词都是通过空格切分)
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
    // or_insert() 返回的是 &mut v， 通过解引用后可以修改 v,
    // 最后达到修改 HashMap 中对应 value 的效果
}
