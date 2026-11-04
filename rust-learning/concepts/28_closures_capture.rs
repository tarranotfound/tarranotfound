fn main() {
    let label = String::from("task");
    let print_label = || println!("label: {label}");
    print_label();

    let mut count = 0;
    let mut increment = || {
        count += 1;
        count
    };

    println!("count: {}", increment());
    println!("count: {}", increment());
}
