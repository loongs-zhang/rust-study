//simple module
pub mod simple_module {
    pub mod host {
        pub fn seat() {
            println!("seat")
        }
    }

    pub mod server {
        pub fn server() {
            println!("server")
        }
    }
}

pub fn call_simple_module() {
    crate::simple_module::host::seat();
    crate::simple_module::server::server();
}