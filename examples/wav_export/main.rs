use wcomm::channels::{Channel, Sound};
use wcomm::modulation::MFSK;
use wcomm::Message;

fn main() {
    let msg = Message::new().data("Hello world!");

    let mod256 = MFSK::new(256).base_frequency(100).delta_frequency(30);
    let channel256 = Sound::new(&mod256);
    channel256.export_wav(&msg, "examples/wav_export/audio/original256.wav", 100);

    let mod16 = MFSK::new(16).base_frequency(100).delta_frequency(100);
    let channel16 = Sound::new(&mod16);
    channel16.export_wav(&msg, "examples/wav_export/audio/original16.wav", 100);
}
