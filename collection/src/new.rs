fn main() {
    let mut arr = Vec::new();
    arr.push(22);
    println!("{:?}", arr);

    let mut arr1 = vec![33, 1, 44];
    arr1.push(22);
    println!("{:?}", arr1);


    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("第三个元素是 {}", third);

    match v.get(9) {
        Some(third) => println!("第三个元素是 {}", third),
        None => println!("去你的第三个元素，根本没有！"),
    }
}
