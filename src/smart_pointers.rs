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
    box_demo03();
    deref_demo01();
    deref_demo02();
    mybox_demo();
    drop_demo();
    rc_demo01();
    rc_demo02();
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
    Cons(i32,Box<List2>),
    Nil
}
fn rc_demo01() {
    let a = List2::Cons(5,
    Box::new(List2::Cons(10,
        Box::new(List2::Nil))));
    let b = List2::Cons(3,Box::new(a));
    //  ^ value used here after move
    // let c = List2::Cons(4,Box::new(a));

}

use std::rc::Rc;
enum List3 {
    Cons(i32,Rc<List3>),
    Nil,
}

fn rc_demo02() {
    let a = Rc::new(List3::Cons(5,Rc::new(List3::Cons(10,Rc::new(List3::Nil)))));
    println!("count after create a:{}",Rc::strong_count(&a));
    let b = List3::Cons(3,Rc::clone(&a));
    println!("count after create b:{}",Rc::strong_count(&a));
    {
        let c = List3::Cons(4,Rc::clone(&a));
        println!("count after create c:{}",Rc::strong_count(&a));
    }
    println!("out of fn count:{}",Rc::strong_count(&a));
   
}

// RefCell<T> 内部可变性 只能用于单线程场景