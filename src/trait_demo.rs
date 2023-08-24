pub fn test01() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {largest}");

    let number_list = vec![10, 50, 30, 90, 40, 20];
    let number = largest_i32(&number_list);
    println!("the largest number is {number}");

    let char_list = vec!['a','d','b','e','g','w'];
    let c = largest_char(&char_list);
    println!("the largest char is {c}");
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list:&[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_generic<T:std::cmp::PartialOrd>(list:&[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}