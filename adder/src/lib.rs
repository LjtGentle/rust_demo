use core::panic;
use std::fmt::format;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn add_two(num: u32) -> u32 {
    num + 2
}

pub fn greeting(name: &str) -> String {
    // format!("hello {}!", name)
    String::from("hello")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater or equal to 1  get:{}.", value)
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100 get:{}.",
                value
            )
        }
        Guess { value }
    }
}

fn println_return_10(num: u32) -> u32 {
    println!("num is {num}");
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn explration() {
        let result = add(3, 4);
        assert_eq!(result, 7);
    }

    // panic 会被标记为失败
    // #[test]
    // fn pannic_demo() {
    //     panic!("call a panic");
    // }

    #[test]
    fn lagest_can_hold_small() {
        let lagest = Rectangle {
            width: 10,
            height: 20,
        };
        let smaller = Rectangle {
            width: 5,
            height: 15,
        };
        assert!(lagest.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_lagest() {
        let lagest: Rectangle = Rectangle {
            width: 10,
            height: 20,
        };
        let small: Rectangle = Rectangle {
            width: 5,
            height: 15,
        };
        assert!(!small.can_hold(&lagest));
    }

    #[test]
    fn it_add_two() {
        let num = 5;
        assert_eq!(7, add_two(num))
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "greet did not contains name,value:{}",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn is_work_result_t_e() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("add not equal four"))
        }
    }

    // 测试输出-pass 不会输出
    // cargo test -- --show-output 执行则就可以输出
    #[test]
    fn return_10_pass() {
        assert_eq!(10, println_return_10(20))
    }

    // 测试输出-failed 会输出
    #[test]
    #[ignore] //被忽略的测试
    fn return_10_failed() {
        assert_eq!(20, println_return_10(30))
    }
    // cargo test -----运行全部测试
    // cargo test 被测试函数名  ----运行指定的单个测试
    // cargo test 被测试函数名的一部分 ---运行包含名字的测试
    // cargo test -- --ignored ---只运行被忽略的代码
}
