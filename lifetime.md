# lifetime

- 生命周期三条消除规则

  - 每一个引用参数都会有自己的的生命周期。

    ```rust
    fn m<'a>(x: &'a i32);
    fn m2<'a, 'b>(x: &'a i32, y: &'b i32);
    ```

  - 如果只有一个输入生命周期（即函数参数只有一个引用类型），那么该生命周期会被赋予所有输出生命周期。

    ```rust
    fn m<'a>(x: &'a i32) -> &i32; // x 的生命周期会被赋予返回值 &i32 
    fn m2<'a>(x: i32, y: &'a i32) -> &i32;
    // y 的生命周期会被赋予返回值 &i32,
    // x 不是引用类型，所有权直接转移到 m2 内，m2 结束后，被销毁（丢弃）
    ```

  - 如果有多个输入生命周期，其中一个为 `&self` 或者 `&mut self`，那么 `&self` 或者 `&mut self` 的生命周期会被赋予所有输出生命周期。

    如果返回值生命周期和 `&self` 或者 `&mut self` 不一样，则需要进行手动标注生命周期。

- 生命周期约束语法
  
  ```rust
  impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
  }
  ```

  标识 `'a: 'b` 告诉编译器 `'a` 不会死在 `'b` 之前。

  > `'a` `'b` 必须声明在同一地方， 如 `impl<'a: 'b, 'b>` `fn<'a: 'b, 'b>`

- 静态生命周期
  
  如字符串字面量 `&str`, `'static` 在程序退出之前都不会被丢弃。

  > :warning: 需要谨慎使用，有可能会导致内存泄漏等问题。

  ```rust
  let s: &'static str = "life is so fucked up huh. So what? I still alive motherfucker.";
  ```

---
