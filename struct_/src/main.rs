#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // .. 语法指定了剩余未显式设置值的字段应有与给定实例对应字段相同的值
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{}", user1.active); // active 不具有所有权
    println!("{}", user1.email); // email 具有所有权，但是未发生转移
    // 下面这行会报错
    // println!("{:?}", user1);

    println!("{:?}", user2.username);
    println!("{:?}", user2.sign_in_count);

    let _black = Color(0, 0, 0);
    let _lat_long_alt = Point(0, 0, 0);
}

// 使用元组结构体表示一些关心值，而不关心字段名称的整体
struct Color (i32, i32, i32); // RGB
struct Point (i32, i32, i32); // 经纬高的点或者 xyz

#[allow(unused)]
struct AlwaysEqual;

// let subject = AlwaysEqual;

// 我们不关心为AlwaysEqual的字段数据，只关心它的行为，
// 因此将它声明为元结构体，然后再为它实现某个特征
// impl SomeTrait for AlwaysEqual {
    
// }

