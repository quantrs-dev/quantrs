fn main() {
    println!("Running SMA example");
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let sma = quantrs::trend::sma(&data, 3);
    println!("{:?}", sma);
}
