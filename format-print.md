# 格式化输出

## 各种宏 `!`

### `print!` & `println!`

输出到标准输出（`stdout`）

### `format!`

格式化文本输出到 `String` 字符串

```rust
    let s1 = format!("{}, world", s);
```

### `eprint!` & `eprintln!`

与 `print!` `println!` 用法一样，但是会输出到标准错误（`stderr`）

## `{:?}` & `{}`

- `{:?}` 实现了 `std::fmt::Debug` 的类型，用于调试输出场景：

    ```rust
    #[derive(Debug)]
    struct User {
        name: String,
        age: i32
    }
    println!("{:?}", User{name: String::from("me"), age:17});
    ```

    除了结构体需要派生 `#[derive(Debug)]`外，其他可直接使用。

- `{}` 实现了 `std::fmt::Display` 的类型，用于自定义格式化输出，友好以展示增加可读性：

  - 未实现 `std::fmt::Display` 也可采用 `{:#?}` 进行格式化输出，但是不能自定义输出格式：

    ```rust
    struct User {
        name: String,
        age: i32
    }
    println!("{:#?}", User{name: String::from("me"), age:17});
    ```

    ```shell
    User {
        name: "me",
        age: 17,
    }
    ```

  - 实现 `std::fmt::Display` 可采用 `{}`

    ```rust  
    use std::fmt;
    impl fmt::Display for User {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "小弟{}, 今年{}", self.name, self.age)
        }
    }
    fn main() {
        let p = User {
            name: "whoami".to_string(),
            age: 18,
        };
        println!("{}", p);
    }
    ```

  - 可以指定位置进行输出

    ```rust
    println!("{1} {0}", 1, 0); // output: 0 1
    ```

  - 起别名进行指定输出

    ```rust
    println!("{last} {first}", first = 1, last = 0); // output: 0 1
    ```

  - 更多可玩性看：
    [格式化参数](https://github.com/yaoming00/rust-course/blob/main/contents/basic/formatted-output.md#%E6%A0%BC%E5%BC%8F%E5%8C%96%E5%8F%82%E6%95%B0)

    宽度，内容填充，精度，对齐，进制，指数，指针地址，转义字符。

- 捕获运行环境中的值进行格式化 ([1.58 新增](https://blog.rust-lang.org/2022/01/13/Rust-1.58.0.html))

  ```rust
  fn new_i32_number() -> i32 {
        3
    }
  
  fn main() {
    const ONE: i32 = 1;
    println!("number: {num}", num = new_i32_number());
    println!("one: {ONE}");
  }
  ```
