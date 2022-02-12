fn function(s: &str) -> (&str, f64) {
    let f64: f64 = match s.trim().parse() {
        Ok(num) => num,
        Err(_) => return (s, f64::NEG_INFINITY),
    };
    return (s, f64);
}

fn main() {
    let (x, y) = function("123");
    println!("{} {}", x, y);
}