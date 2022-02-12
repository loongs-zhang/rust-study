use rust_study::simple_module::Struct;

fn main() {
    let s = Struct::new();
    println!("{}", s.f1);
    //println!("{}", s.f2);
    //                 ^^ private field
}