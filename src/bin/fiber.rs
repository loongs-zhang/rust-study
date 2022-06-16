use std::{ptr};
use std::os::raw::c_void;

pub struct Fiber<F> {
    ///用户函数
    function: Box<F>,
    ///用户参数
    param: Option<*mut c_void>,
}

impl<F> Fiber<F>
    where F: FnOnce(*const c_void, Option<*mut c_void>)
{
    pub fn new(function: F, param: Option<*mut c_void>) -> Self {
        Fiber {
            function: Box::new(function),
            param,
        }
    }

    pub fn call_once(&self) {
        unsafe {
            let fun = ptr::read(self.function.as_ref());
            (fun)(&self as *const _ as *const c_void, self.param);
        }
    }
}

fn main() {
    let fiber = Fiber::new(move |fiber, param| {
        println!("test")
    }, None);
    fiber.call_once();
}