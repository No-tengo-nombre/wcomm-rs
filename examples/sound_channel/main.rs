use wcomm::{Channel, Message, Sound, MFSK};

fn main() {
    let modulation = MFSK::new(16);
    let channel = Sound::new(&modulation);

    // let time = 20;

    // channel.play(500, time);
    // channel.play(1000, time);
    // channel.play(500, time);
    // channel.play(1000, time);
    // channel.play(500, time);
    // channel.play(1000, time);
    // channel.play(500, time);
    // channel.play(1000, time);

    let msg = Message::new().data("Hello world!");
    channel.send(&msg, 100);
}
