#[derive(Debug)]
struct User<'a> {
    account: &'a str,
    password: &'a str,
}

impl User<'static> {
    pub fn new() -> User<'static> {
        User {
            account: "",
            password: "",
        }
    }
}

impl User<'static> {
    pub fn get_account(&self) -> &str {
        self.account
    }
    pub fn set_account(&mut self, new_account: &'static str) -> &mut User<'static> {
        self.account = new_account;
        self
    }
    pub fn get_password(&self) -> &str {
        self.password
    }
    pub fn set_password(&mut self, new_password: &'static str) -> &mut User<'static> {
        self.password = new_password;
        self
    }
}

//tuple struct
struct Color(i32, i32, i32);

fn create_user() {
    let mut u = User::new();
    u.set_account("account")
        .set_password("password");
    println!("{:#?}", u);
}

fn show_user() {
    //可以不按照struce的顺序来声明
    let user = User {
        password: "123456",
        account: "admin",
    };
    println!("{:#?}", user);
}

fn from_user() {
    let user1 = User {
        password: "123456",
        account: "admin",
    };
    let user2 = User {
        password: "password",
        ..user1
    };
    println!("{:#?}", user2);
}

fn main() {
    create_user();
    show_user();
    from_user();
}