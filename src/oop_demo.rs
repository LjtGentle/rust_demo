use crate::oop_demo::oop::{Button, Screen};

mod oop;

/*
    面向对象调用

*/
pub fn demo() {
    test02()
}

fn test01() {
    let mut col: oop::AveragedCollection = Default::default();
    println!("col:{:?}", col);
    col.add(30);
    col.add(20);
    col.add(10);
    let mut average = col.average();
    println!("{}", average);
    let o = col.remove();
    println!("o:{:?}", o);
    average = col.average();
    println!("remove after average:{}", average)
}
fn test02() {
    let  screen =oop::Screen{
        components:vec![
            Box::new(oop::SelectBox{
                width:75,
                height:10,
                options:vec![
                    String::from("yes"),
                    String::from("maybe"),
                    String::from("no")
                ],
            }),
            Box::new(oop::Button{
                width:50,
                height:20,
                label:String::from("okay")
            })
        ],
    };

    screen.run()
}