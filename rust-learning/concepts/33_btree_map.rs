use std::collections::BTreeMap;

fn main() {
    let mut inventory = BTreeMap::new();
    inventory.insert("keyboard", 4);
    inventory.insert("monitor", 2);
    inventory.insert("mouse", 8);

    // BTreeMap iterates over keys in sorted order.
    for (item, quantity) in &inventory {
        println!("{item}: {quantity}");
    }
}
