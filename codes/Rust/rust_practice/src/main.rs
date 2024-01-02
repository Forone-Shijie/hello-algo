fn main() {
    let x = define_x();
    println!("{} world", x);
}

fn define_x() -> String {
    let x = "Hello".to_string();
    x
}
