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
    fn test_4_test() {
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
    }
}
