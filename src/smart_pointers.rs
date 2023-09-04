use std::ops::Deref;
// 在prelude 中可以无需导入
// use std::ops::Drop;
use List::{Cons, Nil};
// enum List {
//     // ---- recursive without indirection
//     Cons(i32,List),
//     Nil,
// }

// box 允许创建递归类型
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn demo() {
    // box_demo02();
    // box_demo03();
    // deref_demo01();
    // deref_demo02();
    // mybox_demo();
    // drop_demo();
    // rc_demo01();
    // rc_demo02();
    // cycle_demo();
    // tree_demo();
    // tree_demo02();
    tree_demo03();
}

// box pointers demo
fn box_demo01() {
    let b = Box::new(5);
    println!("b={}", b);
}

// fn box_demo02() {
//     let list = Cons(1,Cons(2,Cons(3,Nil)));
// }

fn box_demo03() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("list:{:?}", list)
}

fn deref_demo01() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    // ^^^^^^^^^^^^^^^ no implementation for `{integer} == &{integer}`
    // assert_eq!(5,y);
}

fn deref_demo02() {
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn mybox_demo() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    //未实现Deref时 type `MyBox<{integer}>` cannot be dereferenced
    assert_eq!(5, *y);
    println!("y={:?}", y);
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

// 解引用强制转换
fn hello(name: &str) {
    println!("Hello,{}!", name);
}

// drop trait
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    // 该方法不可显示调用
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data)
    }
}

fn drop_demo() {
    let c = CustomSmartPointer {
        data: String::from("hello"),
    };
    //^^^^ explicit destructor calls not allowed
    // c.drop();
    // std::mem::drop 位于prelude，可以不导入包
    // 提早释放
    drop(c);
    let b = CustomSmartPointer {
        data: String::from("world"),
    };
    println!("end of demo fn");
}

// Rc<T> 引用计数智能指针 只能用于单线程的场景
enum List2 {
    Cons(i32, Box<List2>),
    Nil,
}
fn rc_demo01() {
    let a = List2::Cons(5, Box::new(List2::Cons(10, Box::new(List2::Nil))));
    let b = List2::Cons(3, Box::new(a));
    //  ^ value used here after move
    // let c = List2::Cons(4,Box::new(a));
}

use std::rc::Rc;
enum List3 {
    Cons(i32, Rc<List3>),
    Nil,
}

fn rc_demo02() {
    let a = Rc::new(List3::Cons(
        5,
        Rc::new(List3::Cons(10, Rc::new(List3::Nil))),
    ));
    println!("count after create a:{}", Rc::strong_count(&a));
    let b = List3::Cons(3, Rc::clone(&a));
    println!("count after create b:{}", Rc::strong_count(&a));
    {
        let c = List3::Cons(4, Rc::clone(&a));
        println!("count after create c:{}", Rc::strong_count(&a));
    }
    println!("out of fn count:{}", Rc::strong_count(&a));
}

// RefCell<T> 内部可变性 只能用于单线程场景

//RefCell<T> 内部可变模式

// test double

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max: f64 = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error:you are over your quota");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: you've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("warning:you've user up over 75% of your quota!");
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     struct MockMessenger {
//         sent_messages : Vec<String>,
//     }
//     impl MockMessenger {
//         fn new()->MockMessenger {
//             MockMessenger { sent_messages: vec![] }
//         }
//     }

//     impl Messenger for MockMessenger {
//         fn send(&self,msg:&str) {
//             //^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
//             // self.sent_messages.push(String::from(msg));
//         }
//     }

//     #[test]
//     fn it_sends_an_over_75_percent_warning_msg() {
//         let mock_messenger = MockMessenger::new();
//         let mut limit_tracker= LimitTracker::new(&mock_messenger,100);
//         limit_tracker.set_value(80);
//         assert_eq!(mock_messenger.sent_messages.len(),1);
//     }
// }

mod tests {
    use super::*;
    use std::cell::RefCell;
    #[derive(Debug)]
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
            // 在相同作用域，创建了两个可变引用 panic了
            // let mut one_borrow = self.sent_messages.borrow_mut();
            // let mut two_borror = self.sent_messages.borrow_mut();
            // one_borrow.push(String::from(msg));
            // two_borror.push(String::from(msg));
        }
    }
    #[test]
    fn it_sends_an_over_75_percent_warning_msg() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        println!("mock_messenger:{:?}", mock_messenger)
    }
    #[test]
    fn it_sends_less_75_percent() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(60);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 0);
    }
}

use std::cell::RefCell;
use List4::{Cons4, Nil4};

#[derive(Debug)]
enum List4 {
    Cons4(Rc<RefCell<i32>>, Rc<List4>),
    Nil4,
}
fn rc_and_refcell() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons4(Rc::clone(&value), Rc::new(Nil4)));
    let b = Cons4(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons4(Rc::new(RefCell::new(10)), Rc::clone(&a));
    *value.borrow_mut() += 10;
    println!("after a={:?},b={:?},c={:?}", a, b, c);
}

#[derive(Debug)]
enum ListCycle {
    Cons(i32, RefCell<Rc<ListCycle>>),
    Nil,
}

impl ListCycle {
    fn tail(&self) -> Option<&RefCell<Rc<ListCycle>>> {
        match self {
            ListCycle::Cons(_, item) => Some(item),
            ListCycle::Nil => None,
        }
    }
}

fn cycle_demo() {
    let a = Rc::new(ListCycle::Cons(5, RefCell::new(Rc::new(ListCycle::Nil))));
    println!("a initial rc count={}", Rc::strong_count(&a));
    println!("a next item ={:?}", a.tail());

    let b = Rc::new(ListCycle::Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after create b, count={}", Rc::strong_count(&a));
    println!("b initial rc count={}", Rc::strong_count(&b));
    println!("b next item={:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changed a,count={}", Rc::strong_count(&b));
    println!("a rc count after changed a, count={}", Rc::strong_count(&a));
}

// 避免引用循环 将Rc<T>变为Weak<T>

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

fn tree_demo() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });
    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    println!("leaf={:?}", leaf);
    println!("branch={:?}", branch);
}

use std::rc::Weak;
#[derive(Debug)]
struct Node2 {
    value: i32,
    parent: RefCell<Weak<Node2>>,
    children: RefCell<Vec<Rc<Node2>>>,
}

fn tree_demo02() {
    let leaf = Rc::new(Node2 {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf parent={:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node2 {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade((&branch));
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

// 打印count的变化
fn tree_demo03() {
    let leaf = Rc::new(Node2 {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong={},weak={}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node2 {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade((&branch));
        println!(
            "branch strong = {},weak={}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
        println!(
            "leaf strong ={},weak={}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong={},weak={}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
