fn main() {
    let names = ["Ada", "Grace", "Linus"];
    let scores = [98, 95, 91];

    for (index, (name, score)) in names.iter().zip(scores).enumerate() {
        println!("{}. {name}: {score}", index + 1);
    }
}
