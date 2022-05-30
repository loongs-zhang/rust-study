use std::collections::HashMap;

pub struct Hook<T> {
    callback: Box<dyn FnMut(&mut T) -> String>,
}

impl<T> Hook<T> {
    pub fn new(callback: impl FnMut(&mut T) -> String + 'static) -> Self {
        Self {
            callback: Box::new(callback),
        }
    }

    fn run(&mut self, cpu: &mut T) -> String {
        (self.callback)(cpu)
    }
}

pub struct Cpu {
    pub hooks: HashMap<u32, Hook<Cpu>>,
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            hooks: HashMap::new(),
        }
    }

    fn add_hook(&mut self, addr: u32, hook: Hook<Cpu>) {
        self.hooks.insert(addr, hook);
    }

    fn run(&mut self) {
        let mut h = self.hooks.remove(&1).unwrap();
        println!("cpu run:{}", h.run(self));
        self.hooks.insert(1, h);
        self.whatever();
    }

    fn whatever(&self) {
        println!("key:{:?}", self.hooks.keys());
    }
}

pub struct Emu {
    cpu: Cpu,
}

impl Emu {
    fn new() -> Self {
        Emu { cpu: Cpu::new() }
    }

    fn run(&mut self) {
        self.cpu.run();
    }

    fn add_hook(&mut self, addr: u32, hook: Hook<Cpu>) {
        self.cpu.add_hook(addr, hook);
    }
}

fn main() {
    let mut emu = Emu::new();
    {
        let h = Hook::new(|_cpu: &mut Cpu| "a".to_owned());
        emu.add_hook(1, h);
    }
    emu.run();
}