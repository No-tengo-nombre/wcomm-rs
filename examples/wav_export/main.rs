use wcomm::channels::Sound;
use wcomm::modulation::{MFSK, MQAM};
use wcomm::Message;

fn main() {
    let msg = Message::new().data("Peter Piper picked a peck of pickled peppers
A peck of pickled peppers Peter Piper picked
If Peter Piper picked a peck of pickled peppers
Where's the peck of pickled peppers Peter Piper picked?");

    let time = 100;

    let fsk256 = MFSK::new(256).base_frequency(100).delta_frequency(30);
    let channel256 = Sound::new(&fsk256);
    channel256.export_wav(&msg, "examples/wav_export/audio/256fsk.wav", time);

    let fsk16 = MFSK::new(16).base_frequency(100).delta_frequency(100);
    let channel16 = Sound::new(&fsk16);
    channel16.export_wav(&msg, "examples/wav_export/audio/16fsk.wav", time);

    let qam256 = MQAM::new(256, 1000);
    let channel256 = Sound::new(&qam256);
    channel256.export_wav(&msg, "examples/wav_export/audio/256qam.wav", time);

    let qam16 = MQAM::new(16, 1000);
    let channel16 = Sound::new(&qam16);
    channel16.export_wav(&msg, "examples/wav_export/audio/16qam.wav", time);
}
