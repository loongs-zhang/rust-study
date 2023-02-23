#![feature(generators, generator_trait)]

use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

fn main() {
    let mut generator = |arg| {
        println!("generator get {}", arg);
        println!("yield 1");
        yield 1;
        "foo"
    };

    println!("generator started");
    match Pin::new(&mut generator).resume(1) {
        GeneratorState::Yielded(1) => {}
        _ => panic!("unexpected return from resume"),
    }
    match Pin::new(&mut generator).resume(0) {
        GeneratorState::Complete("foo") => {}
        _ => panic!("unexpected return from resume"),
    }
    println!("generator finished");
}