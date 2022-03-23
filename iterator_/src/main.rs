/// 1. 迭代器（iterator）负责遍历序列中的每一项和决定序列何时结束的逻辑。
/// 当使用迭代器时，我们无需重新实现这些逻辑。
///
/// 2. 在 Rust 中，迭代器是 惰性的（lazy），
/// 这意味着在调用方法使用迭代器之前它都不会有效果。
///
/// 3.
/// ```rust
/// fn iterator_demonstration() {
/// let v1 = vec![1, 2, 3];
///
/// let mut v1_iter = v1.iter();
/// // 获取 v1 所有权并返回拥有所有权的迭代器
/// let mut v1_iter2 = v1.into_iter();
/// // 迭代可变引用，前提是 v1 也是可变的
/// let mut v1_iter3 = v1.iter_mut();
///
/// assert_eq!(v1_iter.next(), Some(&1));
/// assert_eq!(v1_iter.next(), Some(&2));
/// assert_eq!(v1_iter.next(), Some(&3));
/// assert_eq!(v1_iter.next(), None);
/// }
/// ```
/// 注意 v1_iter 需要是可变的：在迭代器上调用 next 方法改变了迭代器中用来记录序列位置的状态。
/// 换句话说，代码 消费（consume）了，或使用了迭代器。每一个 next 调用都会从迭代器中消费一个项。
/// 使用 for 循环时无需使 v1_iter 可变因为 for 循环会获取 v1_iter 的所有权并在后台使 v1_iter 可变。
///
/// 4. 从 next 调用中得到的值是 vector 的不可变引用
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    // 迭代器适配器是惰性的，而这里我们需要使用 collect() 消费迭代器。
    // 1. map 方法使用闭包来调用每个元素以生成新的迭代器
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    /*因为 map 获取一个闭包，
    可以指定任何希望在遍历的每个元素上执行的操作。
    这是一个展示如何使用闭包来自定义行为同时又复用 Iterator trait 提供的迭代行为的绝佳例子*/
    assert_eq!(v2, vec![2, 3, 4]);

    // 2. 迭代器的 filter 方法获取一个使用迭代器的每一个项并返回布尔值的闭包。
    // 如果闭包返回 true，其值将会包含在 filter 提供的新迭代器中。
    // 如果闭包返回 false，其值不会包含在结果迭代器中。
    let v3: Vec<i32> = v1.into_iter().filter(|x| *x == 1 || *x == 3).collect();
    assert_eq!(v3, vec![1, 3]);

    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    // 3. zip
    // 将两个迭代器“压缩”成一个对的迭代器。
    // `zip()` 返回一个新的迭代器，它将迭代另外两个
    // 迭代器，返回一个元组，其中第一个元素来自
    // 第一个迭代器，第二个元素来自第二个迭代器。
    //
    // 注意 zip 只产生四对值；理论上第五对值 (5, None) 从未被产生，
    // 因为 zip 在任一输入迭代器返回 None 时也返回 None。
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}

/// 实现 Iterator trait 来创建自定义迭代器
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    /// 定义中唯一要求提供的方法就是 next 方法。
    /// 一旦定义了它，就可以使用所有其他由 Iterator trait 提供的拥有默认实现的方法来创建自定义迭代器了
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
