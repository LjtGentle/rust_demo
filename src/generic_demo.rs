pub fn demo() {
    let v = test02();
    println!("v:{:?}", v);

    let integer = Point { x: 5, y: 10 };
    let float: Point<f64> = Point { x: 3.14, y: 4.56 };
    println!("integer:{:?},float:{:?}", integer, float);
    println!("integer x:{}, float y:{}", integer.x(), float.y());
    println!("float res={}", float.distance_from_origin());

    let both_integer: Point2<i32, i32> = Point2 { x: 3, y: 4 };
    let both_float: Point2<f64, f64> = Point2 { x: 3.14, y: 4.23 };
    let integer_and_float: Point2<i32, f64> = Point2 { x: 23, y: 1.23 };
    println!(
        "both_integer:{:?},both_float:{:?},integer_and_float:{:?}",
        both_integer, both_float, integer_and_float
    );
    let mixp = both_float.mixup(both_integer);
    println!("mixp:{:?}", mixp);
}

fn test02() -> Vec<i32> {
    let list = vec![1, 2, 3, 4, 5, 6];
    list
}

fn largest03<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}
