#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn match01() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    let some_value = Some(0u8);
    match some_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // 只关系3的时候可以用if let
    if let Some(3) = some_value {
        println!("is three")
    }

    // macth 和 if let 之间的选择是特定的环境以及增加简洁度和失去穷尽性检查的权衡取舍

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    match coin {
        Coin::Quarter(state) => println!("state from {:?}", state),
        _ => count += 1,
    }
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = coin {
        println!("if let state from {:?}", state);
    } else {
        count += 1;
    }
}

fn add_fancy_hat() {
    println!("call add_fancy_hat");
}
fn remove_fancy_hat() {
    println!("call remove_fancy_hay");
}
fn move_player(num_spaces: u8) {
    println!("{}", num_spaces);
}

fn reroll() {
    println!("call reroll");
}
