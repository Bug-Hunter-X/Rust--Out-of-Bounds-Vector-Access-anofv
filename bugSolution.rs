fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    match vec.get(10) {
        Some(value) => println!("Value: {}", value),
        None => println!("Index out of bounds"),
    }
} 