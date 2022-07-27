use wcomm::channels::{Channel, Sound};
use wcomm::modulation::MFSK;
use wcomm::Message;

fn main() {
    let modulation = MFSK::new(256).base_frequency(100).delta_frequency(30);
    let channel = Sound::new(&modulation);

    let msg = Message::new().data("Hello world!");
    channel.export_wav(&msg, "examples/wav_export/audio/original.wav", 100);
}
