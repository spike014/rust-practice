# crate & module

- [code](./src) [with comment](../comment.md)

## Crate

### Package

#### new

- 二进制 项目 （可直接运行）入口为：`new-project/src/main.rs`

  ```sh
  cargo new new-project
  ```

- 库 项目 （库类型的 `Package` 只能作为三方库被其它项目引用，不能直接 `cargo run` 运行）

  入口为：`new-project/src/lib.rs`

  ```sh
  cargo new new-lib --lib
  ```

#### 典型的项目结构

**出自** : [包crate - Rust语言圣经(Rust教程 Rust Course)](https://course.rs/basic/crate-module/crate.html#典型的-package-结构)

一个真实项目中典型的 `Package`，会包含多个二进制包，这些包文件被放在 `src/bin` 目录下，
每一个文件都是独立的二进制包，同时也会包含一个库包，该包只能存在一个 `src/lib.rs`：

```css
.
├── Cargo.toml
├── Cargo.lock
├── src
│   ├── main.rs
│   ├── lib.rs
│   └── bin
│       └── main1.rs
│       └── main2.rs
├── tests
│   └── some_integration_tests.rs
├── benches
│   └── simple_bench.rs
└── examples
    └── simple_example.rs
```

- 唯一库包：`src/lib.rs`
- 默认二进制包：`src/main.rs`，编译后生成的可执行文件与`package`同名
- 其余二进制包：`src/bin/main1.rs` 和 `src/bin/main2.rs`，它们会分别生成一个文件同名的二进制可执行文件
- 集成测试文件：`tests` 目录下
- 性能测试 `benchmark` 文件：`benches` 目录下
- 项目示例：`examples` 目录下

这种目录结构基本上是 Rust 的标准目录结构，在 `github` 的大多数项目上，你都将看到它的身影。

## Module

[module -- rust-course](https://course.rs/basic/crate-module/module.html)

### 模块可见性

#### 受限的可见性

```rust
pub mod a {
    pub const I: i32 = 3;

    fn semisecret(x: i32) -> i32 {
        use self::b::c::J;
        x + J
    }

    pub fn bar(z: i32) -> i32 {
        semisecret(I) * z
    }
    pub fn foo(y: i32) -> i32 {
        semisecret(I) + y
    }

    mod b {
        pub(in crate::a) mod c {
            pub(in crate::a) const J: i32 = 4;
        }
    }
}
```

此时，`J` 只对 `a` 可见，其他模块是访问不到。

总结：`pub(crate)` 或 `pub(in crate::a)` 就是限制可见性语法，前者是限制在整个包内可见，后者是通过绝对路径，限制在包内的某个模块内可见：

- `pub` 意味着可见性无任何限制

- `pub(crate)` 表示在当前包可见

- `pub(self)` 在当前模块可见

- `pub(super)` 在父模块可见

- `pub(in <path>)` 表示在某个路径代表的模块中可见，其中 `path` 必须是父模块或者祖先模块

一个很酷的综合例子： [rust-course/use.md at main · rust-course (github.com)](https://github.com/yaoming00/rust-course/blob/main/contents/basic/crate-module/use.md#一个综合例子)
