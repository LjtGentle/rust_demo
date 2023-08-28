pub fn demo() {
    // rc_and_refcell();
    cycle_demo();
}


//RefCell<T> 内部可变模式

// test double

pub trait Messenger {
    fn send(&self,msg:&str);
}

pub struct LimitTracker<'a, T:Messenger> {
    messenger: &'a T,
    value:usize,
    max:usize,
}

impl <'a, T> LimitTracker<'a, T>  where T:Messenger{

    pub fn new(messenger:&T,max:usize) -> LimitTracker<T> {
        LimitTracker { messenger, value: 0, max }
    }

    pub fn set_value(&mut self,value:usize) {
        self.value = value;
        let percentage_of_max: f64 = self.value as f64  / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error:you are over your quota");
        }else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: you've used up over 90% of your quota!");
        }else if percentage_of_max >= 0.75 {
            self.messenger.send("warning:you've user up over 75% of your quota!");
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
        fn new() ->MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self,msg:&str) {
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
        let mut limit_tracker = LimitTracker::new(&mock_messenger,100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(),1);
        println!("mock_messenger:{:?}",mock_messenger)
    }
    #[test]
    fn it_sends_less_75_percent() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger,100);
        limit_tracker.set_value(60);
        assert_eq!(mock_messenger.sent_messages.borrow().len(),0);

    }
}

use std::rc::Rc;
use std::cell::RefCell;
use List::{Cons,Nil};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>,Rc<List>),
    Nil,
}
fn rc_and_refcell() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b =Cons( Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    *value.borrow_mut() += 10;
    println!("after a={:?},b={:?},c={:?}",a,b,c)
;}


#[derive(Debug)]
enum ListCycle {
    Cons(i32,RefCell<Rc<ListCycle>>),
    Nil,
}

impl ListCycle {
    fn tail(&self) -> Option<&RefCell<Rc<ListCycle>>> {
        match self {
            ListCycle::Cons(_,item)=> Some(item),
            ListCycle::Nil => None,
        }
    }
}

fn cycle_demo() {
    let a = Rc::new(ListCycle::Cons(5, RefCell::new(Rc::new(ListCycle::Nil))));
    println!("a initial rc count={}",Rc::strong_count(&a));
    println!("a next item ={:?}",a.tail());

    let b = Rc::new(ListCycle::Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after create b, count={}",Rc::strong_count(&a));
    println!("b initial rc count={}",Rc::strong_count(&b));
    println!("b next item={:?}",b.tail());

    if let Some(link)=a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changed a,count={}",Rc::strong_count(&b));
    println!("a rc count after changed a, count={}",Rc::strong_count(&a));
}