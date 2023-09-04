pub fn iterators_demo() {
    test04();
}

fn test01() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v1_iter = v1.iter();
    for item in v1_iter {
        println!("v:{item}");
    }
}

fn test02() {
    let v1 = vec![1, 2, 3, 4, 5];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    // assert_eq!(v1_iter.next(), Some(&5));
}

fn test03() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let sum: i32 = v1_iter.sum();
    assert_eq!(sum, 6);
}
// 调用迭代器适配器方法 map
fn test04() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}
#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    let in_my_shoes = shoes_in_my_size(shoes, 10);
    assert_eq!(
        in_my_shoes,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}

#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let counters = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b);
    println!("conters:{:?}", counters);
    let counters_filter = counters.filter(|x| x % 3 == 0);
    println!("counters_filter:{:?}", counters_filter);
    let sum: u32 = counters_filter.sum();
    assert_eq!(18, sum)
}
