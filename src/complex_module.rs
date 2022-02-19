pub mod module1;
//重新导出，对用户友好
//原来的写法use rust-study::module1::hello;
//导出后的写法use rust-study::hello;
pub use self::module1::hello;
pub use self::module1::world;

pub mod module2;