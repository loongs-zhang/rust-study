use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    //Rc<T>赋予了数据具备多重所有权的能力
    //RefCell<T>让数据可变
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    //正常来说，不可以从"不可变的引用"借用"可变引用"
    // let x = 5;
    // let y = &mut x;

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));
    let b = List::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = List::Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    println!("value before = {:?}", value);
    *value.borrow_mut() += 10;
    println!("value after borrow_mut = {:?}", value);
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}