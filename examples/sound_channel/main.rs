use wcomm::channels::{Channel, Sound};
use wcomm::modulation::MFSK;

fn main() {
    let modulation = MFSK::new();
    let channel = Sound::new(&modulation);

    let time = 20;

    channel.play(500, time);
    channel.play(1000, time);
    channel.play(500, time);
    channel.play(1000, time);
    channel.play(500, time);
    channel.play(1000, time);
    channel.play(500, time);
    channel.play(1000, time);
}
