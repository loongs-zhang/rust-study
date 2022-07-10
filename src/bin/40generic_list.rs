#[derive(Debug)]
pub struct Node<C, N> {
    current: C,
    next: N,
}

fn main() {
    let end: Node<_, Option<i32>> = Node { current: 1, next: None };
    let middle = Node { current: true, next: end };
    let head = Node { current: "test", next: middle };
    println!("{:#?}", head);
}