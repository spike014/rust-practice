#![allow(unused)]
use std::{fs::File, io::{ErrorKind, Read}, io, fs, net::IpAddr};
use std::error::Error;


fn main() {
    // panic!("Hello, world!");

    // unwrap ：成功则返回值，失败则 panic，总之不进行任何错误处理。
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    println!("{}", home);

    // 查看 Cargo.toml 学习一个编译配置

    /*
    长话短说，如果是 main 线程，则程序会终止，
    如果是其它子线程，该线程会终止，但是不会影响 main 线程。
    因此，尽量不要在 main 线程中做太多任务，将这些任务交由子线程去做，
    就算子线程 panic 也不会导致整个程序的结束。
    */

    let file_with_text: String = String::from("./hello.txt");
    let f = File::open(&file_with_text);
    let _f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Open file ERROR: {:?}", error);
        }
    };

    // 对某些类型的错误进行处理
    let path: String = String::from("./NoSuchFile.txt");
    let f = File::open(&path);
    let _f = match f {
        Ok(file) => file,
        // 对打开文件错误进行处理
        Err(error) => match error.kind() {
            // 文件不存在的错误，创建该文件
            ErrorKind::NotFound => match File::create(&path) {
                Ok(fc) => fc,
                Err(e) => panic!("Creating file ERROR: {:?}", e),
            },
            other_error => panic!("Open file ERROR: {:?}", other_error),
        },
    };

    let _f = File::open(&path).unwrap(); // 遇到错误直接 panic
    // 遇到错误直接 panic, 但是会加上自定义的错误信息，相当于重载了错误打印的函数
    let _f = File::open(&path).expect("Failed to open file");

    let text = read_file_txt(&file_with_text);
    println!("文件内容为: {:?}", text.unwrap());

    println!(
        "v2 读取文件内容为: {:?}",
        read_file_txt_v2(&file_with_text).expect("Read file txt ERROR:")
    );

    println!(
        "v3 读取文件内容为: {:?}",
        read_file_txt_v3(&file_with_text).expect("Read file txt ERROR:")
    );
}

/// 向上传播错误
fn read_file_txt(path: &String) -> Result<String, io::Error> {
    let f = File::open(path);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(res) => {
            println!("读取了 {:?} 个字节", res);
            return Ok(s);
        }
        Err(e) => Err(e),
    }
}

/// ?
/// 从 Golang 过来的，尝尝这个 ?
fn read_file_txt_v2(path: &String) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

/*如果 Result 的值是 Ok，这个表达式将会返回 Ok 中的值而程序将继续执行。
如果值是 Err，Err 中的值将作为整个函数的返回值，提前进行返回，不再执行后面的逻辑
就好像使用了 return 关键字一样，这样错误值就被传播给了调用者。*/

/// 更爽的标准库读取
fn read_file_txt_v3(path: &String) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

// 目前可以理解 Box<dyn Error> 为使用 ? 时 main 允许返回的 “任何类型的错误”。
fn main_2() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}

/*
? 只能用于以下形式：
    let v = xxx()?;
    xxx()?.yyy()?;
*/
