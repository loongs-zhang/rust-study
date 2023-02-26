use genawaiter::{GeneratorState, sync::gen, yield_};

fn main() {
    let x = 10;
    let mut generator = gen!({
        yield_!(x);
        println!("{:?}", backtrace::Backtrace::new());
    });
    let ten = generator.resume();
    assert_eq!(ten, GeneratorState::Yielded(10));
    assert_eq!(generator.resume(), GeneratorState::Complete(()));
}