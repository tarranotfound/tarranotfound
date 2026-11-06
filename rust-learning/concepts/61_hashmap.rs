use std::collections::HashMap;

fn main() {
    let mut inventory = HashMap::new();
    inventory.insert("keyboard", 4);
    inventory.insert("mouse", 7);

    *inventory.entry("monitor").or_insert(0) += 2;

    for (item, quantity) in &inventory {
        println!("{item}: {quantity}");
    }
}