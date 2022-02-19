//FnOnce：捕获外部变量时，取得所有权
//FnMut：捕获外部变量时，可变的借用
//Fn：捕获外部变量时，不可变的借用
struct Cache<T>
    where T: Fn(i32) -> i32
{
    closure: T,
    result: Option<i32>,
}

impl<T> Cache<T>
    where T: Fn(i32) -> i32
{
    fn new(closure: T) -> Self {
        Cache {
            closure,
            result: None,
        }
    }

    fn result(&mut self, arg: i32) -> i32 {
        match self.result {
            Some(v) => v,
            None => {
                //注意这里的调用方式，不是self.closure(arg)
                let result = (self.closure)(arg);
                self.result = Some(result);
                return result;
            }
        }
    }
}

fn main() {
    let mut cache = Cache::new(|num| num);
    println!("{}", cache.result(1));
    println!("{}", cache.result(2));
}