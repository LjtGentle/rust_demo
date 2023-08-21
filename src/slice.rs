// slice是没有所有权的类型
pub fn slice01() {
    let s1 = String::from("hello word");
    let i1 = first_word(&s1);
    let s2 = String::from("hello");
    let i2 = first_word(&s2);
    println!("i1:{i1},i2:{i2}");

    let s = String::from("hello world");
    // 左闭右开
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello:{hello},world:{world}");
    let s1 = &s[..5];
    let s2 = &s[6..];
    println!("s1:{s1},s2:{s2}");
    let word = first_word02(&s);
    println!("word:{word}");
    // cannot borrow as mutable
    // s.clear();

    let s = "hello world";
    let word = frist_word03(s);
    println!("word:{word}");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word02(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn frist_word03(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
