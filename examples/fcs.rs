fn main() {
    let args: Vec<u8> = std::env::args()
        .skip(1)
        .map(|arg| u8::from_str_radix(&arg, 16).expect("Invalid bytes!"))
        .collect();
    let fcs = args.into_iter().reduce(|x, y| x ^ y).expect("Empty!");
    println!("fcs: {:02x}", fcs);
}
