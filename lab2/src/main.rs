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

    let node = b.get(&KEY);
    match node {
        None => println!("Node with key 6 not found"),
        Some(n) => println!("Value of node with key 6 is: {}", n.borrow().value),
    }

    let node = b.get(&KEY);
    if let Some(n) = node {
        println!(
            "If borrowed value was returned then you should see it here: {}",
            n.borrow().value
        );
    }

    println!("Starting from 6 iter is:");
    for (key, value) in b.find(&KEY) {
        println!("Key: {}, Value: {}", key, value);
    }

    println!("Full BST:");
    for (key, value) in b {
        println!("Key: {}, Value: {}", key, value);
    }
}
