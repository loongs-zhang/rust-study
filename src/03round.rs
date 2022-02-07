pub fn round() {
    round_loop();
    round_while();
    round_for();
}

fn round_loop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop The result is {}", result);
}

fn round_while() {
    let mut counter = 0;
    while counter != 10 {
        counter += 1;
    };
    println!("while The result is {}", counter * 2);
}

fn round_for() {
    let mut counter = 0;
    for index in 0..10 {
        counter += 1;
    }
    println!("for The result is {}", counter * 2);
}