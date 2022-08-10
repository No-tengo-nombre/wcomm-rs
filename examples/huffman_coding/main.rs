use wcomm::{HuffmanCoding, Message, SourceCoding};

fn main() {
    let small_msg = Message::from_string("Hello world!", "HEADER");
    let encoder = HuffmanCoding::from_message(&small_msg);
    let small_enc = encoder.encode(&small_msg);
    let small_dec = encoder.decode(&small_enc, 0);
    println!("{}", encoder);
    println!("{} -> DECODED : {}", small_msg, small_dec);
    println!(
        "\n=== SIZES ===\n{} b -> ENCODED : {} b -> DECODED : {} b",
        small_msg.bit_size(),
        small_enc.bit_size(),
        small_dec.bit_size()
    );

    println!("\n\n");

    let msg1 = Message::from_string(
        "Peter Piper picked a peck of pickled peppers
    A peck of pickled peppers Peter Piper picked
    If Peter Piper picked a peck of pickled peppers
    Where's the peck of pickled peppers Peter Piper picked?",
        "THIS IS A TEST HEADER",
    );
    let encoder1 = HuffmanCoding::from_message(&msg1);
    let msg1_enc = encoder1.encode(&msg1);
    let msg1_dec = encoder1.decode(&msg1_enc, 0);
    println!("{}", encoder);
    println!("{} -> DECODED : {}", msg1, msg1_dec);
    println!(
        "\n=== SIZES ===\n{} b -> ENCODED : {} b -> DECODED : {} b",
        msg1.bit_size(),
        msg1_enc.bit_size(),
        msg1_dec.bit_size()
    );
}
