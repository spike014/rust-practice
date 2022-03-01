#![allow(unused)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
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
    fn it_works_should_fail() -> Result<(), String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    // 不能对这些使用 Result<T, E> 的测试使用 #[should_panic] 标注。
    // 相反应该在测试失败时直接返回 Err 值。
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