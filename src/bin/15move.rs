fn main() {
    let x = vec![1, 2, 3];
    let y = vec![1, 2];
    //move可以强制闭包取得环境值(这里是x)的所有权
    let closure = move |param| param == x;
    println!("{:?}", closure(y));
    //println!("{:?}", x);
    //                 ^ value borrowed here after move
}