use std::fs::File;
use std::io::{Error, Read};

//可恢复的错误
fn recoverable(i: i8) -> Result<String, &'static str> {
    if i > 0 {
        return Result::Ok(i.to_string());
    }
    return Result::Err("error");
}

fn spread_error(i: i8) -> Result<String, &'static str> {
    let f = File::open("test.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err("can not open test.txt"),
    };
    let mut s = String::new();
    return match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err("read test.txt failed"),
    };
}

fn spread_error_cannot_simplify(i: i8) -> Result<String, &'static str> {
    let f = File::open("test.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err("can not open test.txt"),
    };
    let mut s = String::new();
    return match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err("read test.txt failed"),
    };
}

fn spread_error_can_simplify(i: i8) -> Result<String, Error> {
    let f = File::open("test.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    return match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    };
}

fn spread_error_simplified(i: i8) -> Result<String, Error> {
    // ?借助了std::convert::From::from()实现
    // ?只适用于返回Result的函数
    let mut f = File::open("test.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    return Ok(s);
}

fn main() {
    println!("{:#?}", recoverable(1));
    println!("{:#?}", recoverable(-1).expect("test error"));
    //启动时设置 RUST_BACKTRACE=1 会打印详细错误信息
    //panic!不可恢复，因此不推荐使用
    panic!("crash");
}