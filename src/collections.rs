pub fn vector01() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    println!("vec={:?}", v);
    let third = &v[2];
    println!("third:{third}");
    match v.get(10) {
        Some(third) => println!("three is {}", third),
        None => println!("thess is no three"),
    }
    // pannic
    // let does_not_exist = &v[100];
    let exist = &v[1];
    let does_not_exist = v.get(100);

    // 不可变引用与可变引用的作用交替
    // let first = &v[0];
    // v.push(40);
    // println!("first is {first}");

    let v = vec![1, 5, 9];
    for i in &v {
        println!("i:{i}");
    }
    let mut v = vec![2, 4, 6];
    for i in &mut v {
        *i += 50;
    }
    println!("v:{:?}", v);

    let row = vec![
        SpreadsheetCell::Int(30),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text(String::from("hello world")),
    ];
    println!("row:{:?}", row);
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
