fn main() {
    //variable in rust
    // unsigned integer
    // u8, u16, u32, u64, u128

    let unsigned = 10;

    // signed integer
    // 1B, 116, 132,

    let signed = -10;

    // float is used for decimals 1.4

    let float = 1.023;

    println!(
        "Unsigned : {}, Signed: {}, Float : {}",
        unsigned, signed, float
    );

    // char - can only be
    let letter = "c";
    let emoji = "\u{1F600}";

    println!("letter : {}, Emoji : {}", letter, emoji);
}
