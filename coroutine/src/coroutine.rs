use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::c_void;
use std::rc::{Rc};
use context::stack::{ProtectedFixedSizeStack};
use crate::scheduler::Scheduler;

static mut id: usize = 0;

#[derive(Debug, Copy, Clone)]
enum State {
    //已创建
    Created,
    //运行中
    Running,
    //被挂起
    Suspend,
    //结束
    Finished,
}

#[derive(Debug)]
pub struct Coroutine<'a> {
    //协程id
    id: usize,
    //协程的运行时栈
    stack: ProtectedFixedSizeStack,
    //协程调用的函数
    function: fn(i32) -> i32,
    //协程参数
    param: Option<*mut c_void>,
    //协程上下文
    context: HashMap<*mut (), *mut ()>,
    //调度此协程的调度器
    scheduler: Rc<RefCell<&'a mut Scheduler<'a>>>,
    //协程当前状态
    state: State,
}

impl<'a> Coroutine<'a> {
    //创建协程
    pub fn new(function: fn(i32) -> i32, scheduler: &'a mut Scheduler<'a>) -> Result<Rc<Self>, &str>

    {
        let scheduler = Rc::new(RefCell::new(scheduler));
        let coroutine = Coroutine {
            id: unsafe {
                let temp = id + 1;
                id = temp;
                temp
            },
            //默认2KB
            stack: match ProtectedFixedSizeStack::new(2048) {
                Ok(s) => s,
                Err(_) => return Err("failed to apply stack"),
            },
            function,
            param: Option::None,
            context: HashMap::new(),
            scheduler: Rc::clone(&scheduler),
            state: State::Created,
        };
        let mut scheduler = (*scheduler).borrow_mut();
        let index = scheduler.add_coroutine(coroutine);
        match scheduler.get_coroutine(index) {
            Some(s) => Ok(Rc::clone(&s)),
            None => Err("coroutine not exists"),
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    //Created -> Running
    pub fn resume(&self) {}

    //挂起
    pub fn r#yield(&self, param: *mut c_void) {
        //保存当前协程的栈

        //主协程会被继续执行
    }
}