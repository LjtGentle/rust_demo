use std::fmt::Display;

pub fn demo() {
    // 静态生命周期
    let s: &'static str = "I have a rustcc";
    let tweet = Tweet {
        username: String::from("gentle"),
        content: String::from("i like rust"),
        reply: true,
        retweet: true,
    };
    println!("1 new tweet: {}", tweet.sunmarize());
    println!("2 tweet author:{}", tweet.sunmarize_author02());
    println!("2 tweet:{}", tweet.sunmarize02());
    // notify(tweet);
    notify02(tweet);

    let t = Text {};
    println!("1 text:{}", t.sunmarize());

    let article = NewsArticle {
        headline: String::from("吸引眼球的标题"),
        location: String::from("sz"),
        author: String::from("gentle"),
        content: String::from("xxxxx-xxxx"),
    };
    println!("1 news:{}", article.sunmarize());
    println!("get author:{}", article.sunmarize_author());
    // notify(article);
    notify02(article);

    //^^^^^^^^^^^ the trait `Sunmary` is not implemented for `SimpleText`
    // let simple_text = SimpleText {};
    // notify02(simple_text);

    let number_list = vec![10, 4, 50, 6, 7, 35];
    let n_largest = largest(&number_list);

    let char_list = vec!['a', 'j', 'b', 'z', 'g', 'q'];
    let c_largest = largest(&char_list);
    println!("n:{n_largest},c:{c_largest}");
    let article = NewsArticle {
        content: String::from("xxx-xxx"),
        ..Default::default()
    };
    println!("article:{:?}", article);
    test_live04();
}

pub trait Sunmary {
    fn sunmarize(&self) -> String {
        String::from("Read more....")
    }
    fn sunmarize_author(&self) -> String {
        String::from("defualt-gentle")
    }
}

struct SimpleText {}
struct Text {}

impl Sunmary for Text {}

#[derive(Default, Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Sunmary for NewsArticle {
    fn sunmarize(&self) -> String {
        format!("{}, by{} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Sunmary for Tweet {
    fn sunmarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Sunmary02 {
    fn sunmarize_author02(&self) -> String;
    fn sunmarize02(&self) -> String {
        format!("Read more from:{}...", self.sunmarize_author02())
    }
}

impl Sunmary02 for Tweet {
    fn sunmarize_author02(&self) -> String {
        format!("@{}", self.username)
    }
}

// trait 作为参数
pub fn notify(item: impl Sunmary) {
    println!("bearking news! {}", item.sunmarize());
}
// trait bound 语法糖
pub fn notify02<T: Sunmary>(item: T) {
    println!("bound--breaking news! {}", item.sunmarize());
}

// 参数 指定多个trait
pub fn notify03(item: impl Sunmary + Display) {}

// trait bound 指定多个trait
pub fn notify04<T: Sunmary + Display>(item: T) {}

pub fn notify05<T: Display + Clone, U: Clone + Default>(t: T, u: U) -> u32 {
    0
}
// 通过where简化 trait bound
pub fn notify06<T, U>(t: T, u: U) -> u32
where
    T: Display + Clone,
    U: Clone + Default,
{
    0
}

pub fn get_sunmarize() -> impl Sunmary {
    Tweet {
        username: String::from("gentle"),
        content: String::from("xxxx-xxxx"),
        reply: true,
        retweet: false,
    }
}

// todo fix
// pub fn get_sunmarize_by_type(sunmary_type: u8) -> impl Sunmary {

//     if sunmary_type ==1 {
//         Tweet {
//             username: String::from("gentle"),
//             content: String::from("xxxx-xxxx"),
//             reply: true,
//             retweet: false,
//         }
//     }else {
//         Text{}
//     }
// }

fn largest<T: PartialOrd + Copy>(list: &[T]) -> &T {
    let mut big = &list[0];
    for item in list {
        if item > big {
            big = item;
        }
    }
    big
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmd_display(&self) {
        if self.x >= self.y {
            println!("in pair big number is x:{}", self.x)
        } else {
            println!("in pair big number is y:{}", self.y)
        }
    }
}

// 返回值 ^ expected named lifetime parameter
// fn longest(x :&str,y:&str) -> &str {
//     if x.len() >= y.len() {
//         x
//     }else {
//         y
//     }

// }

// result 的生命周期是取 x y中最短的
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

// fn test_live() {
//     let n;
//     {
//         let x= 5;
//         //^^ borrowed value does not live long enough
//         n = &x;
//         println!("in....n:{n}");
//     }
//     println!("out....n:{n}");
// }

fn test_live02() {
    let str1 = String::from("hello");
    {
        let str2 = String::from("world123");
        let res = longest(str1.as_str(), str2.as_str());
        println!("res:{}", res)
    }
}

// fn test_live03 () {
//     let str1 = String::from("hello");
//     let res;
//     {
//         let str2 = String::from("world123");
//         // 第二个参数生命周期不够长
//         //^^^^^^^^^^^^^ borrowed value does not live long enough
//         res = longest(str1.as_str(), str2.as_str());
//     }
//     println!("res:{}",res);
// }

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn test_live04() {
    let novel = String::from("call me inshmael. Some year ago...");
    let first_sentence = novel.split(".").next().expect("Could not found a .");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    let part = i.announce_and_return_part("123");
    println!("i:{:?},part:{part}", i)
}

// 生命周期被省略了
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn rule_two(s: &str) -> &str {
    s
}

// 规则3
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("attention please: {}", announcement);
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("ann is {}", ann);
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}
