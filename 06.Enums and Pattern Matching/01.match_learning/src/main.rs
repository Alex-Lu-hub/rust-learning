fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five: {:#?}", five);
    println!("six: {:#?}", six);
    println!("none: {:#?}", none);

    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(1) => println!("one"),
        Some(3) => println!("three"),
        Some(5) => println!("five"),
        Some(7) => println!("seven"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("not three");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
