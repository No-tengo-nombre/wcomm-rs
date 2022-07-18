use wcomm::Message;

fn main() {
    let msg = Message::new().data(&"Hello world!").header(&"HEADER");
    println!("MESSAGE {}", msg);
    println!("BIT SIZE {}", msg.bit_size());
    println!("HEADER {}", msg.header_as_string());
    println!("DATA {}", msg.data_as_string());
}
