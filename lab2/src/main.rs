mod map;

use map::Map;

const KEY: i32 = 6;

fn main() {
    let mut a: Map<i32, &str> = Map::new();

    println!("Is empty: {}", a.is_empty());

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

    a.insert(6, "https://olejka.ru/ss/six.jpg");

    for (key, value) in a.iter() {
        println!("Key: {}, Value: {}", key, value);
    }

    let b = a.clone();
    drop(a);

    println!("Is empty: {}", b.is_empty());

    let value = b.get(&KEY);
    match value {
        None => println!("Node with key {} not found", KEY),
        Some(v) => println!("Value of node with key {} is: {}", KEY, v),
    }

    let value = b.get(&KEY);
    if let Some(v) = value {
        println!(
            "If borrowed value's ownership was returned, then you should see it here: {}",
            v
        );
    }

    println!("Starting from {} iter is:", KEY);
    for (key, value) in b.find(&KEY) {
        println!("Key: {}, Value: {}", key, value);
    }

    println!("Full BST:");
    for (key, value) in b {
        // At this point b is moved and destroyed
        println!("Key: {}, Value: {}", key, value);
    }
}
