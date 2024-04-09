use core::panic;

fn rot13(message: &str) {
    static ALPHABET: [&str; 26] = [
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M",
        "N", "O", "P", "Q", "R","S", "T", "U", "V", "W", "X", "Y", "Z",
    ];

    // let mut rot_msg: Vec<_> = Vec::new();
    let message_chars: Vec<_> = message.chars().filter_map(|c| Some(c.is_ascii_alphabetic())).collect();
    println!("{:?}\n{}", message_chars, message);


    // let r =
}

fn main() {

rot13("abc1222 hello 1");
}
