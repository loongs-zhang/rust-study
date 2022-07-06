use async_trait::async_trait;
use deadpool::managed;

#[derive(Debug)]
enum Error { Fail }

struct Computer {}

impl Computer {
    async fn get_answer(&self) -> i32 {
        42
    }
}

struct Manager {}

#[async_trait]
impl managed::Manager for Manager {
    type Type = Computer;
    type Error = Error;

    async fn create(&self) -> Result<Computer, Error> {
        Ok(Computer {})
    }

    async fn recycle(&self, _: &mut Computer) -> managed::RecycleResult<Error> {
        //这里可以根据Computer的内置属性来判断是否拒绝回收
        Ok(())
    }
}

type Pool = managed::Pool<Manager>;

#[tokio::main]
async fn main() {
    let mgr = Manager {};
    let pool = Pool::builder(mgr).build().unwrap();
    let mut conn = pool.get().await.unwrap();
    let answer = conn.get_answer().await;
    assert_eq!(answer, 42);
    let mut conn = pool.get().await.unwrap();
    let answer = conn.get_answer().await;
    assert_eq!(answer, 42);
}