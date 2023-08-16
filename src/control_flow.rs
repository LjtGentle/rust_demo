
pub fn test01() {
    let number = 5;
    if number > 4 {
        println!("大于4")
    }else {
        println!("小于4")
    }
    let flag = true;
    if flag {
        println!("flag is true");
    }
    //xpected `bool`, found integer
    // if number {

    // }
    if number != 0 {
        println!("number not is zero");
    }
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    }else if number%3 == 0 {
        println!("number is divisible by 3");
    }else if number%2== 0 {
        println!("number is divisble by 2");
    }

    let condition = true;
    let number = if condition {3} else {4};
    println!("number is {number}");
    // ^^^ expected integer, found `&str`
    // let number = if condition {3} else {"4"};
    //死循环
    // loop {
    //     println!("run....")
    // }

    // loop label
    let mut count = 0;
    'counting_loop:loop {
        println!("count:{count}");
        let mut remaining = 10;
        loop {
            println!("remaining:{remaining}");
            if remaining ==9 {
                break;
            }
            if count == 2 {
                break 'counting_loop;
            }
            remaining -= 1;
        }
        count+=1;
    }
    println!("end... count:{count}");

    // loop return
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter*2;
        }
    };
    println!("result:{result}");
    //while
    let mut number = 10;
    while number >0  {
        println!("while ... number={number}");
        number -= 2;
    }
    // loop collection 
    let a = [10,20,30,40,50];
    let mut index = 0;
    while index < 5 {
        println!("e={}",a[index]);
        index += 1;
    }
    for e in a {
        println!("in  e={e}");
    }
    for number in (1..4).rev() {
        println!("number is {}",number);
    }
}