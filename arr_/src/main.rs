fn main() {
    let a = [1, 3, 4];
    println!("{:?}", a);

    let a1: [i64; 4] = [1, 3, 4, 3];
    println!("{:?}", a1);

    // slice
    let slice: &[i64] = &a1[1..3];
    // let slice: &[i64] = &a1[1..6]; panic
    println!("{:?}", slice);
}
