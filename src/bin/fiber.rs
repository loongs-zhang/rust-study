use std::any::Any;
use std::mem;
use std::os::raw::c_void;

#[derive(Copy, Clone)]
pub struct Fiber<F>
    where F: FnOnce(*const c_void, Option<*mut c_void>) + Copy
{
    ///用户函数
    function: F,
    ///用户参数
    param: Option<*mut c_void>,
}

impl<F> Fiber<F>
    where F: FnOnce(*const c_void, Option<*mut c_void>) + Copy
{
    pub fn new(function: F, param: Option<*mut c_void>) -> Self {
        Fiber {
            function,
            param,
        }
    }

    pub fn call_once(&self) {
        (self.function)(&self as *const _ as *const c_void, self.param);
    }
}

fn main() {
    let fiber = Fiber::new(move |fiber, param| {
        println!("test")
    }, None);
    fiber.call_once();
}