//! 测试模块注释

#[doc(alias = "hhhh")] // 给类型定义别名，以便在文档中快速搜索（准备命中时可实现快速跳转）
pub mod hosting {
    /// `add_to_waitlist` 添加到等待名单 （测试函数注释）
    pub fn add_to_waitlist() {
        println!("waiting...")
    }

    /// `add_one` 将指定值加
    ///
    /// # Examples
    ///
    /// ```
    /// use restaurant::hosting;
    /// let arg = 6;
    /// # println!("这里在文档看不到，但是会在测试的时候运行。因为在开头加上了 # ");
    /// # // 这个注释也是看不到的因为加上了 # 被隐藏了起来。
    /// assert_eq!(hosting::add_one(arg), 7);
    /// ```
    /// # Panics
    ///
    /// ```should_panic
    /// panic!("这里造成了恐慌，但是希望可以使用标记 should_panic 来通过这次测试。")
    /// ```
    ///
    /// # Errors
    ///
    /// error 1
    ///
    /// # Safety
    ///
    /// safety 1
    /// 试试跳转到标准库 [`Option`]
    /// 试试跳转到自己的代码 [`crate::eat_at_restaurant`] （使用完整路径）
    /// [`self::Bar`]
    ///
    #[doc(alias = "111111")]
    pub fn add_one(x: i32) -> i32 {
        x + 1
    }

    /// 同名项使用 struct@ fn@ !  (! 表示跳转到同名宏)
    /// 跳转到结构体  [`Foo`](struct@Foo)
    /// [`Self::test_comment()`] 跳转该结构体实现的 test_comment 方法
    /// ```
    /// 以上 跳转到方法 (VSCode 使用 rust-analyzer 插件暂不可以，文档内可以)
    /// ```
    pub struct Bar;

    impl Bar {
        pub fn test_comment() -> i32 {
            9
        }
    }

    /// 跳转到同名函数 [`Foo`](fn@Foo) // 私有，在文档不可跳转，但是在代码里可以
    struct Foo {}

    /// 跳转到同名宏 [`foo!`]
    fn Foo() {}

    macro_rules! foo {
        () => {};
    }
}
