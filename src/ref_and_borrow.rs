// 在任意给定的时候，要么只能有一个可变引用，要么只能有多个不可变引用
pub fn references_and_Bborrowing01() {
    let s = String::from("hello");
    let len = calculate_len(&s);
    println!("s:{s},len:{len}");
    // 可变引用 与不可变引用
    // change(&s)
    let mut s = String::from("hello");
    changeMut(&mut s);
    println!("mut s:{s}");

    //data race 数据竞争的产生原因（需要三个行为同时发生才会造成数据竞争）:
    // 两个或更多指针同时访问同一数据
    // 至少有一个指针被用来写入数据
    // 没有同步数据访问的机制
    let mut s = String::from("hello");
    let r1 = &mut s;
    // ^^^^^^ second mutable borrow occurs here
    // let r2 = &mut s;

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    //  ^^^^^^ mutable borrow occurs here
    let r3 = &mut s;
    //  -- immutable borrow later used here
    // println!("{},{},{}",r1,r2,r3);

    // 一个引用的作用域从声明的地方开始持续到最后一次使用为止。
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("r1:{r1},r2:{r2}");
    let r3 = &mut s;
    println!("r3:{r3}");
    // let ret = dangle();
    let ret = no_dangle();
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

//  expected named lifetime parameter
// 悬垂引用
// fn dangle() -> &String {
//     let s = String::from("hello");
//     return &s;
// }

fn no_dangle() ->String {
    let s = String::from("hello");
    s
}