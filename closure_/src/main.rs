use std::thread;
use std::time::Duration;
use std::collections::HashMap;

struct Cacher<T>
    where T: Fn(u32) -> u32
/*储存到 Cacher 实例的 calculation 字段的闭包必须有一个 u32 参数
（由 Fn 之后的括号的内容指定）并必须返回一个 u32（由 -> 之后的内容）*/
{
    calculation: T,
    value: HashMap<u32, u32>,
}
/*
字段 value 是 Option<u32> 类型的。在执行闭包之前，value 将是 None。
如果使用 Cacher 的代码请求闭包的结果，
这时会执行闭包并将结果储存在 value 字段。
接着如果代码再次请求闭包的结果，这时不再执行闭包，
而是会返回存放在 HashMap 中的结果。*/

impl<T> Cacher<T>
    where T: Fn(u32) -> u32 {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: HashMap::new(),
            }
        }

        fn get_value(&mut self, key: &u32) -> u32 {
            match self.value.get(key) {
                None => {
                    let v = (self.calculation)(*key);
                    self.value.insert(*key, v);
                    v
                }
                Some(i) => {
                    *i
                }
            }
        }
    }

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    // 如果你希望强制闭包获取其使用的环境值的所有权，可以在参数列表前使用 move 关键字。
    // 这个技巧在将闭包传递给新线程以便将数据移动到新线程中时最为实用。
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}", x); 错误，上一行中所有权已经被转移

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

fn generate_workout(intensity: u32, random_number: u32) {
    // 定义一个闭包
    // let expensive_closure = |num| {
    // // let expensive_closure = |num: u32| -> u32 {
    // // 可以为闭包标注类型（一般无必要）
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num+1
    // };
    /*
    注意这个 let 语句意味着 expensive_closure 包含一个匿名函数的定义，
    不是调用匿名函数的 返回值。
    回忆一下使用闭包的原因是我们需要在一个位置定义代码，储存代码，
    并在之后的位置实际调用它；
    期望调用的代码现在储存在 expensive_closure 中。
    
    定义了闭包之后，可以改变 if 块中的代码来调用闭包以执行代码并获取结果值。
    调用闭包类似于调用函数；
    指定存放闭包定义的变量名并后跟包含期望使用的参数的括号。
    */
    
    let mut expensive_closure = Cacher::new( |num| {
        // let expensive_closure = |num: u32| -> u32 {
        // 可以为闭包标注类型（一般无必要）
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num+1
        });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure.get_value(&intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure.get_value(&intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.get_value(&intensity)
            );
        }
    }
}

// 闭包并不用于这样暴露在外的接口：
// 他们储存在变量中并被使用，不用命名他们或暴露给库的用户调用。

// 闭包体只有一行，可以省略 {}
/*
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
*/

// 如果尝试对同一闭包使用不同类型则会得到类型错误。

// 闭包可以捕获其环境并访问其被定义的作用域的变量。

/*
- FnOnce
    消费从周围作用域捕获的变量，闭包周围的作用域被称为其 环境，environment。
    为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。
    其名称的 Once 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。
- FnMut 获取可变的借用值所以可以改变其环境
- Fn 从其环境获取不可变的借用值*/
