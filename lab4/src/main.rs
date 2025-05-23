mod hash_map;

use hash_map::HashMap;

fn main() {
    let mut map: HashMap<&str, &str> = HashMap::default();
    // let mut map: HashMap<&str, &str> = HashMap::new(1);

    println!("Is empty: {}", map.is_empty());

    map.insert("one", "In russian it's \"один\"");
    println!("Is empty: {}", map.is_empty());
    println!("Curent length: {}", map.len());
    println!("Curent load factor: {}", map.load_factor());

    map.insert("phone", "+1 555 987 65 43");
    map.insert("alert", "Уведомление о выселении");

    println!("Curent length: {}", map.len());
    println!("Curent load factor: {}", map.load_factor());

    map.remove(&"one");

    println!("Curent length: {}", map.len());
    println!("Curent load factor: {}", map.load_factor());
    println!("Value for \"phone\": {}", map[&"phone"]);

    println!("Curent max load factor is: {}", map.max_load_factor());
    map.set_max_load_factor(4.0);
    println!("Curent max load factor is: {}", map.max_load_factor());

    map.clear();
}
