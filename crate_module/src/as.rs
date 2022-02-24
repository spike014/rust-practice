use std::fmt::Result;
use std::io::Result as IoResult; // 使用 as 给 引入项起别名

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
