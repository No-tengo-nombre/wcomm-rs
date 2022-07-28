use wcomm::channels::{Channel, Sound};
use wcomm::modulation::MFSK;
use wcomm::Message;

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
