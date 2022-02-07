use std::ops::RangeInclusive;
include!("01function.rs");
include!("02if_else.rs");
include!("03round.rs");
include!("04ownership.rs");
include!("guess.rs");
include!("request.rs");

fn main() {
    println!("Hello, world!");
    call_function();
    if_else();
    round();
    ownership();
    // guess();
    test();
}
