mod map;

use map::Map;

fn main() {
    let mut map: Map<i32, &str> = Map::new();

    map.insert(1, "one");
    map.insert(-2, "minus two");
    map.insert(3, "three");
    map.insert(6, "six");
    map.insert(6, "https://olejka.ru/ss/six.jpg");

    let node = map.find(&6);

    match node {
        None => println!("Node with key 6 not found"),
        Some(n) => println!("Value of node with key 6 is: {}", n.borrow().value),
    }

    let node = map.find(&6);
    if let Some(n) = node {
        println!(
            "If borrowed value was returned then you should see it here: {}",
            n.borrow().value
        );
    }
}
