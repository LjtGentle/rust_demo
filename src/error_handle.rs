use core::panic;
use std::io;
use std::{
    fs::File,
    io::{ErrorKind, Read},
};

pub fn test01() {
    // call_panic();
    // open_file02();
    // open_file_panic02();
    let res = read_username_from_file03();
    println!("res={:?}", res);
}

fn call_panic() {
    panic!("my crash and abort")
}

fn call_pannic02() {
    let v = vec![1, 2, 3];
    v[90];
}

fn open_file01() {
    let filename = "hello.txt";
    let f = File::open(filename);
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(fc) => fc,
                Err(e) => panic!("create file err={:?}", e),
            },
            _ => panic!("open file err={:?}", error),
        },
    };
}

fn open_file02() {
    let filename = "hello.txt";
    let f = File::open(filename).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(filename).unwrap_or_else(|error| {
                panic!("create file err={:?}", error);
            })
        } else {
            panic!("open file errr={:?}", error);
        }
    });
}

fn open_file_panic() {
    let filename = "hello.txt";
    let f = File::open(filename).unwrap();
}

fn open_file_panic02() {
    let filename = "hello.txt";
    let f = File::open(filename).expect("open file failed");
}

// 错误传递
fn read_username_from_file() -> Result<String, io::Error> {
    let filename = "hello.txt";
    let f = File::open(filename);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
// 错误传递简写
fn read_username_from_file02() -> Result<String, io::Error> {
    let filename = "hello.txt";
    let mut f = File::open(filename)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file03() -> Result<String, io::Error> {
    let filename = "hello.txt";
    let mut s = String::new();
    File::open(filename)?.read_to_string(&mut s)?;
    Ok(s)
}
