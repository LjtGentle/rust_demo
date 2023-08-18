pub fn ownership01() {
    // 离开作用域变量失效
    let s = "hello";
    {
        let s = "hello word";
        println!("s is {s}");
    }
    println!("s is {s}");

    let s = String::from("string hello");
    println!("s is {s}");

    let mut s = String::from("hello,");
    s.push_str("world");
    println!("mut s is {s}");
    // move test
    let x = 5;
    let y = x;
    println!("x is {x}");
    println!("y is {y}");
    let s1 = String::from("hello123");
    let s2 = s1;
    println!("s2 is {s2}");
    //  value borrowed here after move
    // println!("s1 is {s1}");
    let s3 = s2.clone();
    println!("s3 is {s3}, s2 is {s2}");

    let ss = String::from("ssss");
    takes_ownership(ss);
    let number: u32 = 19;
    makes_copy(number);

    println!("number is {number}");
    //value borrowed here after move
    // println!("ss is {ss}");

    //Return Values and Scope
    let s1 = give_ownership();
    let s2 = String::from("sss22");
    let s3 = token_give_ownership(s2);
    println!("s1:{s1}");
    //value borrowed here after move
    // println!("s2:{s2}");
    println!("s3:{s3}");
}

// value borrowed here after move
fn takes_ownership(s: String) {
    println!("in fn s is {s}");
}

fn makes_copy(i: u32) {
    println!("in fn i is {i}");
}

fn give_ownership() -> String {
    let ss = String::from("infnsss");
    ss
}

fn token_give_ownership(s: String) -> String {
    println!("in fn s:{s}");
    return s;
}
