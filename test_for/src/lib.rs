#![allow(unused)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn add_two(n: i32) -> i32 {
    n + 2
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}

pub struct Guess {
    value: i32,
}

#[cfg(test)]
mod tests {
    // 测试
    use super::*;

    #[test] // 使用此标志表明该函数为测试函数
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    #[ignore]
    fn test_4_test_will_fail_final() {
        assert_eq!(1, 1);               // equal
        // assert_eq!(1, 2); failed

        assert!(!false);                // true
        // assert!(false); failed

        assert_ne!(false, true);        // not equal
        // assert_ne!(false, false); failed
        // 在代码按预期运行，我们不确定值 会 是什么，
        // 不过能确定值绝对 不会 是什么的时候，这个宏最有用处。
        // 如果一个函数保证会以某种方式改变其输出，
        // 这时最好的断言可能就是函数的输出不等于其输入。

        /* assert_eq! 和 assert_ne! 宏在底层分别使用了 == 和 !=。
        当断言失败时，这些宏会使用调试格式打印出其参数，
        这意味着被比较的值必需实现了 PartialEq 和 Debug trait。
        所有的基本类型和大部分标准库类型都实现了这些 trait。
        对于自定义的结构体和枚举，需要实现 PartialEq 才能断言他们的值是否相等。
        需要实现 Debug 才能在断言失败时打印他们的值。
        因为这两个 trait 都是派生 trait，如下所提到的，
        ```
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        fn main() {
            let rect1 = Rectangle {
                width: 30,
                height: 50,
            };

            println!("rect1 is {:?}", rect1);
        }
        ```
        通常可以直接在结构体或枚举上添加 #[derive(PartialEq, Debug)]*/

        // 自定义失败信息，类似于 format! 输出
        assert!(false, "[自定义信息]===》This test failed. Get errcode: {}", 500)
    }


    // 使用 should_panic 检查 panic
    // #[should_panic] 属性位于 #[test] 之后，对应的测试函数之前。
    #[test]
    #[should_panic]
    fn test_should_panci() {
        panic!("I panic here.")
    }
    /*
    should_panic 测试结果可能会非常含糊不清，因为它只是告诉我们代码并没有产生 panic。
    should_panic 甚至在一些不是我们期望的原因而导致 panic 时也会通过。
    为了使 should_panic 测试结果更精确，
    我们可以给 should_panic 属性增加一个可选的 expected 参数。*/
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    // #[should_panic(expected = "Guess value must be greater than or equal to 1")]
    //    ^
    //  failed
    fn greater_than_100() {
        Guess::new(200);
    }
    // expected 信息的选择取决于 panic 信息有多独特或动态，和你希望测试有多准确。
    // So rusty!


    #[test]
    #[ignore]
    fn it_works_should_fail() -> Result<(), String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    // 不能对这些使用 Result<T, E> 的测试使用 #[should_panic] 标注。
    // 相反应该在测试失败时直接返回 Err 值。


    // 将测试线程设置为 1，告诉程序不要使用任何并行机制。
    // 这也会比并行运行花费更多时间，不过在有共享的状态时，测试就不会潜在的相互干扰了。
    // $ cargo test -- --test-threads=1


    // 显示打印输出
    // $ cargo test -- --nocapture
    // 类似于 go test -v
    // ps： 实测（cargo 1.58.0 (f01b232bc 2022-01-19)）单元测试没有彩色打印输出

    // 运行指定测试
    // 1. 例如，运行 fn test_xxxx() {} 这一测试函数
    // $ cargo test test_xxxx
    // 类似于 go test -timeout 10m -run ^Testxxxx$
    //
    // 2. 此命令支持前缀匹配模块名，测试函数名
    // 存在 test_111 test_222 test111 三个测试
    // $ cargo test test_
    // 将会运行 test_111 test_222 这两个测试
    //
    // 存在 tests1 tests222 test_111 三个
    // $ cargo test test_
    // 将会运行 test_111 test_222 这两个测试
    //
    // 还支持 $ cargo test tests::test_ 这种形式的前缀匹配

    // 忽略某些测试
    /*
        #[test]
        #[ignore]
        fn expensive_test() {
            // 需要运行一个小时的代码
        }
    */
    // 此时 cargo test 会显示 test result: ... 1 ignored; ...
    //
    // 只运行被忽略的测试
    // $ cargo test -- --ignored
}


// 测试组织结构指南
/*
    1. 【单元测试】与他们要测试的代码共同存放在位于 src 目录下相同的文件中。
       规范是在每个文件中创建包含测试函数的 tests 模块，并使用 cfg(test) 标注模块。
      
       测试模块的 #[cfg(test)] 标注告诉 Rust 只在执行 cargo test 时才编译和运行测试代码，
       而在运行 cargo build 时不这么做。

    2. 在 Rust 中，【集成测试】对于你需要测试的库来说完全是外部的。
        同其他使用库的代码一样使用库文件，也就是说它们只能调用一部分库中的公有 API 。
        集成测试的目的是测试库的多个部分能否一起正常工作。
        一些单独能正确运行的代码单元集成在一起也可能会出现问题，所以集成测试的覆盖率也是很重要的。
        为了创建集成测试，你需要先在项目根目录创建一个 tests 目录，与 src 同级。
        Cargo 会将每一个文件当作单独的 crate 来编译。

        2.1 运行指定集成测试
         $ cargo test --test integration_test

        2.2 帮助函数
            在 tests 下新建 helper.rs 文件，编写可见的相关函数。
            为了不让 helper 出现在测试输出中，我们将创建 tests/common/mod.rs ，
            而不是创建 tests/common.rs 。
            这是一种 Rust 的命名规范，这样命名告诉 Rust 不要将 common 看作一个集成测试文件。

            ps: 
            将 helper.rs 修改为 common/helper.rs 后，cargo test 报错：
            to create the module `common`, create file "tests/common.rs" or "tests/common/mod.rs"
            也就是说 common 下文件名必须是 mod.rs

          2.2.1 二进制 crate 的集成测试
              如果项目是二进制 crate 并且只包含 src/main.rs 而没有 src/lib.rs，
              这样就不可能在 tests 目录创建集成测试并使用 extern crate 导入 src/main.rs 中定义的函数。
              只有库 crate 才会向其他 crate 暴露了可供调用和使用的函数；二进制 crate 只意在单独运行。
  
              为什么 Rust 二进制项目的结构明确采用 src/main.rs 调用 src/lib.rs 中的逻辑的方式？
              因为通过这种结构，集成测试 就可以 通过 extern crate 测试库 crate 中的主要功能了，
              而如果这些重要的功能没有问题的话，src/main.rs 中的少量代码也就会正常工作且不需要测试。
              ps: 当初你总觉得用的很不爽，但是后来都明白爸妈的好了吧。 So rusty~
*/