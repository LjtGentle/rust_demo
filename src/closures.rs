use std::thread;
use std::time::Duration;
// use std::(thread,time::Duration);

pub fn closures_demo() {
    // let num = simulated_expensive_calculation(18);
    // println!("get number:{num}");

    let simluated_user_speified_value = 10;
    let simluated_random_number = 3;
    generate_workout04(simluated_user_speified_value, simluated_random_number);
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    // sleep 2 secs
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "today,do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("take a break today! remember to stay hydrated!")
        } else {
            println!(
                "today run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

// 简单化了调用次数，缺点就是不需要执行的条件也执行了改函数
fn generate_workout02(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!("today, do {} pushups", expensive_result);
        println!("next, do {} siteups!", expensive_result);
    } else {
        if random_number == 3 {
            println!("take a break today! remember to stay hydarated!")
        } else {
            println!("today run for {} minutes", expensive_result);
        }
    }
}

// 闭包，问题跟函数调用一样
fn generate_workout03(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!("today, do {} pushsup!", expensive_closure(intensity));
        println!("next, do {} siteups", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("take a break today! remember to stay hydarated!");
        } else {
            println!("today run for {} minutes", expensive_closure(intensity));
        }
    }
}

// 闭包结构体解决问题
fn generate_workout04(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("today, do {} pushsup!", expensive_result.value(intensity));
        println!("next, do {} siteups", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("take a break today! remember to stay hydarated!");
        } else {
            println!(
                "today run for {} minutes",
                expensive_result.value(intensity)
            );
        }
    }
}

// 缺点调用时传入不同的值，获取到的结果都是跟第一次一样
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculate: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculate: T) -> Cacher<T> {
        Cacher {
            calculate,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculate)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
