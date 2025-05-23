mod hash_map;
use hash_map::HashMap;

fn main() {
    let mut map: HashMap<&str, &str> = HashMap::new(16);

    println!("Is empty: {}", map.is_empty());
    println!("Curent load factor is: {}", map.max_load_factor());

    map.insert("one", "In russian it's \"один\"");
    map.insert("phone", "+1 555 987 65 43");
    map.insert("alert", "Уведомление о высилении");

    println!("Is empty: {}", map.is_empty());
    println!("Curent length: {}", map.len());

    map.remove(&"one");

    println!("Curent length: {}", map.len());
    println!("Value for \"phone\": {}", map[&"phone"]);

    map.set_max_load_factor(4.0);
    println!("Curent load factor is: {}", map.max_load_factor());

    map.clear();
}
