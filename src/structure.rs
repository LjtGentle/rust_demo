#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sigin_in_count: u64,
}

// 所有权报错
// TODO fix
// struct UserInfo {
//     active:bool,
//expected named lifetime parameter
//     username:&str,
//     email:&str,
//     sigin_in_count: u64,
// }

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct NoField;

pub fn struct01() {
    let u1 = User {
        email: String::from("mine@qq.com"),
        username: String::from("gentle"),
        active: true,
        sigin_in_count: 123456,
    };
    println!("u1:{:?}", u1);
    let mut u2 = User {
        email: String::from("mine@qq.com"),
        username: String::from("gentle"),
        active: true,
        sigin_in_count: 123456,
    };
    u2.username = String::from("gentle123");
    println!("u2:{:?}", u2);

    let email = String::from("1223@qq.com");
    let username = String::from("Ben");
    let u3 = build_user(email, username);
    println!("u3:{:?}", u3);
    let u4 = build_user02(String::from("123@qq.com"), String::from("Ben"));
    println!("u4:{:?}", u4);

    let u5 = User {
        active: u4.active,
        username: u4.username,
        email: u4.email,
        sigin_in_count: 5,
    };
    println!("u5:{:?}", u5);

    let u6 = User {
        sigin_in_count: 6,
        ..u5
    };
    println!("u6:{:?}", u6);
    let back = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("back:{:?}", back);
    println!("origin:{:?}", origin);

    let st = NoField;
    println!("st:{:?}", st);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sigin_in_count: 2,
    }
}
// 同名字段省略
fn build_user02(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sigin_in_count: 3,
    }
}

pub fn user_sturct() {
    let w = 30;
    let h = 40;
    println!("area:{}", area(w, h));

    let d = (31, 42);
    println!("area:{}", area02(d));

    let r = Rectangle {
        width: 32,
        heigth: 42,
    };
    println!("r:{:?}", r);
    dbg!(&r);
    println!("area:{}", area03(r));

    let r = Rectangle {
        width: 30,
        heigth: 40,
    };
    println!("area:{},width:{}", r.area(), r.width());
    let r2 = Rectangle {
        width: 25,
        heigth: 35,
    };
    let can = r.can_hold(&r2);
    println!("can={can}");

    let square = Rectangle::square(10);
    println!("square:{:?}", square);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.heigth * self.width
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.heigth >= other.heigth
    }
    //关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            heigth: size,
        }
    }
}

fn area(w: u32, h: u32) -> u32 {
    return w * h;
}

fn area02(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area03(r: Rectangle) -> u32 {
    r.heigth * r.width
}
