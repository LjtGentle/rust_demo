// 变量不可变
pub fn test01() {
    let x = 5;
    println!("{x}");
    let x = 6;
    println!("{x}");
    //cannot assign twice to immutable variable
    // x = 7;
    // println!("{x}");
}

// mut 为可变变量， 可变
pub fn test02() {
    let mut x = 5;
    println!("{x}");
    x = 6;
    println!("{x}");
}

const MY_CONSTANTS: u32 = 60 * 60 * 3;

pub fn test03() {
    println!("{MY_CONSTANTS}");
    //cannot assign to this expression
    // MY_CONSTANTS = 30;
}

pub fn shadowing() {
    let x = 1;
    println!("{x}");
    {
        // 离开作用范围被删除
        let x = 2;

        println!("{x}");
    }
    println!("{x}");
}

pub fn shadowing02() {
    // 重新定义变量spaces
    let spaces = "   ";
    println!("111->{spaces}");
    let spaces = spaces.len();
    println!("111->{spaces}");

    // 赋值报错
    let mut spaces2 = "  ";
    println!("222->{spaces2}");
    //^^^^^^^^^^^^^ expected `&str`, found `usize`
    // spaces2 = spaces2.len();
    // println!("{spaces2}");
}

pub fn data_type() {
    // 必须显示定义出类型
    let guess: u32 = "42".parse().expect("not a number");
    println!("guess is {guess}");
    let x = 2.0;
    let y: u32 = 4;
    let sum = 3 + 4;
    let sub = 5 - 9;
    let mul = 3.14 * 5.0;
    let division = 9 / 2;
    let remainder = 9 % 2;
    println!("x is {x},y is {y}, sum is {sum},sub is{sub}, mul is {mul},division is {division}, remainder is {remainder}");
    let t = true;
    let f = false;
    println!("t is {t},f is {f}");
    let c: char = 'c';
    let heart_eyed_cat = '😻';
    println!("c is {c}, heart_eyed_cat is {heart_eyed_cat}");
    let tup: (i32, bool, f32) = (3, true, 3.14);
    println!("tup is {:?}", tup);
    let array = [1, 2, 3, 4, 5];
    let one = array[0];
    let two = array[1];
    println!("one is {one}, two is {two}");
    println!("array is {:?}", array);
    let array2 = [3, 5];
    println!("array2 is {:?}", array2);
}
