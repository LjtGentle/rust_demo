pub fn references_and_Bborrowing01() {
    let s = String::from("hello");
    let len = calculate_len(&s);
    println!("s:{s},len:{len}");
    // 可变引用 与不可变引用
    // change(&s)
    let mut s = String::from("hello");
    changeMut(&mut s);
    println!("mut s:{s}");
}

fn calculate_len(s: &String) -> usize {
    return s.len();
}

//s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
// fn change(s :&String) {
//     s.push_str("123");
// }

fn changeMut(s: &mut String) {
    s.push_str("11122233");
}
