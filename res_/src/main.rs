use std::{fs::File, io::{ErrorKind, Read}, io, fs};

fn main() {
    let file_with_text: String = String::from("./hello.txt");
    let f = File::open(&file_with_text);
    let _f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Open file ERROR: {:?}", error);
        },
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
                Err(e) => panic!("Creating file ERROR: {:?}", e)
            },
            other_error => panic!("Open file ERROR: {:?}", other_error)
        },
    };

    let _f = File::open(&path).unwrap(); // 遇到错误直接 panic
    // 遇到错误直接 panic, 但是会加上自定义的错误信息，相当于重载了错误打印的函数
    let _f = File::open(&path).expect("Failed to open file"); 

    let text = read_file_txt(&file_with_text);
    println!("文件内容为: {:?}", text.unwrap());


    println!("v2 读取文件内容为: {:?}",
        read_file_txt_v2(&file_with_text).expect("Read file txt ERROR:"));

    println!("v3 读取文件内容为: {:?}",
        read_file_txt_v3(&file_with_text).expect("Read file txt ERROR:"));
}

/// 向上传播错误
fn read_file_txt(path: &String) -> Result<String, io::Error> {
    let f = File::open(path);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(res) => {
            println!("读取了 {:?} 个字节", res);
            return Ok(s)
        }
        Err(e) => Err(e)
    }
}

/// ? 
/// 从 Golang 过来的，尝尝这个 ?
fn read_file_txt_v2(path: &String) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

/// 更爽的标准库读取
fn read_file_txt_v3(path: &String) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

/* 
? 只能用于以下形式：
    let v = xxx()?;
    xxx()?.yyy()?;
*/
