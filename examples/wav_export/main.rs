use wcomm::channels::{Channel, Sound};
use wcomm::modulation::MFSK;
use wcomm::Message;

fn main() {
    let msg = Message::new().data("Tu eres menso");

    let time = 100;

    let fsk256 = MFSK::new(256).base_frequency(100).delta_frequency(30);
    let channel256 = Sound::new(&fsk256);
    channel256.export_wav(&msg, "examples/wav_export/audio/256fsk.wav", time);

    let fsk16 = MFSK::new(16).base_frequency(100).delta_frequency(100);
    let channel16 = Sound::new(&fsk16);
    channel16.export_wav(&msg, "examples/wav_export/audio/16fsk.wav", time);
}
