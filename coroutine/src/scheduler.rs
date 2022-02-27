use std::cell::RefCell;
use std::ops::Index;
use std::rc::Rc;
use context::stack::ProtectedFixedSizeStack;
use crate::coroutine::Coroutine;

#[derive(Debug)]
pub struct Scheduler<'a>
{
    //主协程
    main: Option<Box<Coroutine<'a>>>,
    //当前存活协程数
    alive: Rc<RefCell<u16>>,
    //所有协程
    coroutines: Vec<Option<Rc<Coroutine<'a>>>>,
}

impl<'a> Scheduler<'a> {
    pub fn new() -> Self {
        Scheduler {
            main: Option::None,
            alive: Rc::new(RefCell::new(0)),
            coroutines: vec![],
        }
    }

    pub fn add_coroutine(&mut self, coroutine: Coroutine<'a>) -> usize {
        let id = coroutine.get_id();
        self.coroutines[id] = Some(Rc::new(coroutine));
        id
    }

    pub fn get_coroutine(&self, id: usize) -> &Option<Rc<Coroutine<'a>>> {
        self.coroutines.get(id)
            .unwrap()
    }
}