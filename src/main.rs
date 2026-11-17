use string_literals::*;

fn main() {
    println!("{}", is_empty(""));
    println!("{}", is_ascii("rust"));
    println!("{}", contains("rust", "d"));
    println!("{:?}", split_at("rust", 2));
    println!("{}", find("rust", 'u'));
}