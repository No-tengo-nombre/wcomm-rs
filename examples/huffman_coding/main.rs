use wcomm::encoding::source::{HuffmanCoding, SourceCoding};
use wcomm::Message;

fn main() {
    let small_msg = Message::from_string("Hello world!", "HEADER");
    let encoder = HuffmanCoding::from_message(&small_msg);
    let small_enc = encoder.encode(&small_msg);
    let small_dec = encoder.decode(&small_enc, 0);
    println!("{}", encoder);
    println!(
        "{} -> DECODED : {}",
        // small_msg.as_binary(), small_dec.as_binary()
        small_msg, small_dec
    );
}
