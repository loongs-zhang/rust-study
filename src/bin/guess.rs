use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        //第一个变量guess被第二个变量隐藏了。这意味着我们随后使用这个名称时，
        //它指向的将会是第二个变量。我们可以重复使用let关键字并配以相同的名称来不断地隐藏变量
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                //非法输入，需要重新输入
                println!("Input must be a number!");
                continue;
            }
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                //猜测成功，跳出循环
                break;
            }
        }
    }
}