pub fn demo() {
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

pub fn notify02<T: Sunmary>(item: T) {
    println!("bound--breaking news! {}", item.sunmarize());
}
