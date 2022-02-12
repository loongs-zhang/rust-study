#[derive(Debug)]
enum Kind<'a> {
    IPv4(u8, u8, u8, u8),
    IPv6(&'a str),
}

fn main() {
    println!("{:#?}", Kind::IPv4(127, 0, 0, 1));
    println!("{:#?}", Kind::IPv6("0:0:0:0:0:0:0:1"));
}