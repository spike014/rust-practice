# 注释

## 文档注释 *

- 文档注释需要位于 `lib` 类型的包中，例如 `src/lib.rs` 中

- 支持 `Markdown` 语法，

  - 可以划分标题，代码支持高亮，
  - 进行 `Doc test` 文档测试

- 相关命令：

  - 生成文档

    ```sh
    $ cargo doc
    ```

  - 在浏览器查看文档

    ```sh
    $ cargo doc --open
    ```

  - 运行测试（包括文档测试 【文档注释里面的测试代码】）：

    ```sh
    $ cargo test
    ```

    > 只会运行位于 `lib` 类型包（包括被 `lib` 类型包使用的模块）的文档测试，其他的文档测试不会运行。

## 包和模块级别的注释

- 需要卸载文件**最顶部**

- 包级别的注释也分为两种：行注释 `//!` 和块注释 `/*! ... */`

> front_of_house.rs 文件如下:

```rust
//! 测试模块注释

pub mod hosting {
    /// `add_to_waitlist` 添加到等待名单 （测试函数注释）
    pub fn add_to_waitlist() {
        println!("waiting...")
    }
}
```

## 例子

- [个人笔记例子](./crate_module/src/front_of_house.rs)

- [from rust-course (github.com)](https://github.com/yaoming00/rust-course/blob/main/contents/basic/comment.md#一个综合例子)

