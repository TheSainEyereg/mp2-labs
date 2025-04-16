mod map;

use map::Map;

const KEY: i32 = 6;

fn main() {
    let mut a: Map<i32, &str> = Map::new();

    a.insert(8, "eight");
    a.insert(-2, "minus two");
    a.insert(3, "three");
    a.insert(10, "ten");
    a.insert(1, "one");
    a.insert(6, "six");
    a.insert(14, "fourteen");
    a.insert(4, "four");
    a.insert(7, "seven");
    a.insert(13, "thirteen");

    for (key, value) in a.iter() {
        println!("Key: {}, Value: {}", key, value);
    }

    a.insert(6, "https://olejka.ru/ss/six.jpg");

    let mut b = a.clone();

    a.clear();

    println!("Is \"a\" empty: {}", a.is_empty());

    drop(a);

    println!("Is \"b\" empty: {}", b.is_empty());

    let value = b[KEY];
    println!("Value of node with key {} is: {}", KEY, value);

    println!("Starting from {} iter is:", KEY);
    for (key, value) in b.find(&KEY).unwrap() {
        println!("Key: {}, Value: {}", key, value);
    }

    b.remove(&KEY);

    println!("Full BST:");
    for (key, value) in b {
        // At this point b is moved and destroyed
        println!("Key: {}, Value: {}", key, value);
    }
}
