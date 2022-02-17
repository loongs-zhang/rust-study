fn dangling_reference() {
    // let r3;
    {
        let s = String::from("hello");
        // r3 = &s;
        //      ^^ borrowed value does not live long enough
    }
    // println!("{}", r3);
}

// 'a生命周期标注x、y和返回值的生命周期一样长
// 注意'a生命周期实际取的是x、y中生命周期最短的那个
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 'static表示静态生命周期，会一直持续到应用停止
// 因此除非必要，否则尽量不使用
fn longer2(x: &'static str, y: &'static str) -> &'static str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    println!("{}", longer("1", "12"));
}