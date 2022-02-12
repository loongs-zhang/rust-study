//simple module
pub mod simple_module {
    pub mod host {
        pub fn seat() {
            println!("seat")
        }
    }

    pub mod server {
        pub fn server() {
            //super只能访问上1级的内容
            super::super_method();
            super::super::super_method();
            println!("server")
        }
    }

    fn super_method() {
        println!("super");
    }

    pub struct Struct {
        //加pub使得外部可以访问
        pub f1: u32,
        f2: u32,
    }

    impl Struct {
        pub fn new() -> Self {
            Struct {
                f1: 1,
                f2: 2,
            }
        }
    }
}

fn super_method() {
    println!("super super");
}

pub fn call_simple_module() {
    crate::simple_module::host::seat();
    crate::simple_module::server::server();
}