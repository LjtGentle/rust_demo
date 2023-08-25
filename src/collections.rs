use std::collections::HashMap;

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

    let mut s = String::new();
    let data = "init content";
    s = "init content".to_string();
    println!("s:{s},data:{data}");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s:{s}");

    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    println!("s:{s}");

    let mut s = String::from("lo");
    s.push('l');
    println!("s:{s}");

    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s2:{s2},s3:{s3}");
    // value borrowed here after move
    // println!("s1:{s1}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s:{s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s:{s}");

    // 索引字符串
    let s1 = String::from("hello");
    //  ^^^^^ `String` cannot be indexed by `{integer}`
    // let h = s1[0];

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s:{s}");

    let s = String::from("hello,world!");
    for c in s.chars() {
        println!("{c}");
    }
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    let mut scores = HashMap::new();
    scores.insert("one", 1);
    scores.insert("two", 2);
    println!("scores:{:?}", scores);

    let teams = vec![String::from("Bule"), String::from("Yellow")];
    let inital_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(inital_scores.iter()).collect();
    println!("scores:{:?}", scores);

    // 哈希map的所有权
    let field_name = String::from("Favorite color");
    let field_value = String::from("Bule");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("map:{:?}", map);
    // ^^^^^^^^^^^ value borrowed here after move
    // println!("{field_name},{field_value}");

    let mut scores = HashMap::new();
    scores.insert("bule", 10);
    scores.insert("yello", 50);
    let bule = scores.get("bule");
    println!("bule:{:?}", bule);

    scores.insert("bule", 25);
    scores.insert("red", 5);
    for (key, value) in scores {
        println!("key:{key},value:{value}");
    }
    // 上面的遍历拿走了所有权
    // ^^^^^^ value borrowed here after move
    // println!("scores{:?}",scores);

    // 根据旧值更新
    let mut map = HashMap::new();
    let text = "hello world wonderful world";
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("map:{:?}", map);
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
