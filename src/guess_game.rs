use rand::Rng;
use std::cmp::Ordering;
use std::io;

/* 直接打印输入 */
pub fn guess_game01() {
    println!("Guess the number!");
    println!("please input your number:");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    println!("your guess is:{guess}");
}

// 打印随机数和收入
pub fn guess_game_random01() {
    println!("Guess th number!");
    println!("please input your number.");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    println!("your guess is {guess},secret_number is {secret_number}");
}

// 用std的枚举进行比较 随机数和输入
pub fn guess_game_random02() {
    println!("please input your number.");
    let secret_number = rand::thread_rng().gen_range(1..101).to_string();
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    println!("guess is {guess},secret_number is {secret_number}");
    match guess.cmp(&secret_number) {
        Ordering::Equal => {
            println!("your win!");
        }
        Ordering::Greater => {
            println!("too big!");
        }
        Ordering::Less => {
            println!("to small!");
        }
    }
}

// loop 比较直到相等
pub fn guess_game_random03() {
    print!("please input your number.");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        let guess_number: u32 = guess.trim().parse().expect("please type a number!");
        println!("guess_number is {guess_number}, secret_number is {secret_number}");
        match guess_number.cmp(&secret_number) {
            Ordering::Equal => {
                println!("your win!");
                break;
            }
            Ordering::Greater => {
                println!("too big! try again!")
            }
            Ordering::Less => {
                println!("too small! try again!")
            }
        }
    }
    println!("out of process");
}
