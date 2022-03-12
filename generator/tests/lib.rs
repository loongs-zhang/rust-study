#![allow(deprecated)]
#![allow(unused_assignments)]

extern crate generator;

use generator::*;

#[test]
fn test_return() {
    let mut g = Gn::new_scoped(|_s| {
        return 42;
    });
    assert_eq!(g.next(), Some(42));
    assert!(g.is_done());
}

#[test]
fn generator_is_done1() {
    let mut g = Gn::new_scoped(|mut s| {
        s.yield_(2);
        done!();
    });

    assert_eq!(g.next(), Some(2));
    assert!(!g.is_done());
    assert_eq!(g.next(), None);
    assert!(g.is_done());
}

#[test]
fn generator_is_done_with_drop() {
    let mut g = Gn::new_scoped(|mut s| {
        s.yield_(String::from("string"));
        done!();
    });

    assert_eq!(g.next(), Some(String::from("string")));
    assert!(!g.is_done());
    assert_eq!(g.next(), None);
    assert!(g.is_done());
}

#[test]
fn test_scoped() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let x = Rc::new(RefCell::new(10));

    let x1 = x.clone();
    let mut g = Gn::<()>::new_scoped_local(move |mut s| {
        *x1.borrow_mut() = 20;
        s.yield_with(());
        *x1.borrow_mut() = 5;
    });

    g.next();
    assert!(*x.borrow() == 20);

    g.next();
    assert!(*x.borrow() == 5);

    assert!(g.is_done());
}

#[test]
fn test_scoped_yield() {
    let mut g = Gn::new_scoped(|mut s| {
        let mut i = 0;
        loop {
            let v = s.yield_(i);
            i += 1;
            match v {
                Some(x) => {
                    // dbg!(x, i);
                    assert_eq!(x, i);
                }
                None => {
                    // for elegant exit
                    break;
                }
            }
        }
        20usize
    });

    // start g
    g.raw_send(None);

    for i in 1..100 {
        let data: usize = g.send(i);
        assert_eq!(data, i);
    }

    // quit g
    g.raw_send(None);
}

#[test]
fn test_inner_ref() {
    let mut g = Gn::<()>::new_scoped(|mut s| {
        use std::mem;
        // setup something
        let mut x: u32 = 10;

        // return interal ref not compiled becuase the
        // lifetime of interal ref is smaller than the generator
        // but the generator interface require the return type's
        // lifetime bigger than the generator

        // the x memory remains on heap even returned!
        // the life time of x is assosiated with the generator
        // however modify this interal value is really unsafe
        // but this is useful pattern for setup and teardown
        // which can be put in the same place
        // s.yield_(&mut x);
        s.yield_(unsafe { mem::transmute(&mut x) });

        // this was modified by the xvoker
        assert!(x == 5);
        // teardown happened when the generator get dropped
        // this is just a safe dummy ret
        static mut RET: u32 = 0;
        unsafe { &mut RET }
    });

    // use the resource setup from generator
    let a = g.next().unwrap();
    assert!(*a == 10);
    *a = 5;
    // a keeps valid until the generator dropped
}

#[test]
fn test_scope_gen() {
    // now we can even deduce the input para type
    let mut g = Gn::new_scoped(|mut s| {
        let i = s.yield_(0).unwrap();
        // below would have a compile error, nice!
        // s.yield_(Box::new(0));
        i * 2
    });

    assert_eq!(g.raw_send(None), Some(0));
    assert_eq!(g.raw_send(Some(3)), Some(6));
    assert_eq!(g.raw_send(None), None);
}

#[test]
fn test_scope_yield_from_send() {
    let mut g = Gn::new_scoped(|mut s| {
        let g1 = Gn::new_scoped(|mut s| {
            let mut i: u32 = s.yield_(1u32).unwrap();
            i = s.yield_(i * 2).unwrap();
            i * 2
        });

        let i = s.yield_from(g1).unwrap();
        // here the return type should be 0u32
        i * 2
    });

    let n = g.send(3);
    assert_eq!(n, 1);
    let n = g.send(4);
    assert_eq!(n, 8);
    let n = g.send(10);
    assert_eq!(n, 20);
    // the last send has no meaning for the return
    let n = g.send(7);
    assert!(n == 14);
    assert!(g.is_done());
}

#[test]
#[should_panic]
fn done_in_normal() {
    done!();
}
