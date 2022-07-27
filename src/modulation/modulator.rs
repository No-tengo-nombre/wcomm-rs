use crate::channels::Channel;
use crate::Message;

pub trait Modulator {
    fn get_name(&self) -> String;
    fn get_sampling_frequency(&self) -> u32;
    fn send_msg(&self, channel: &dyn Channel, msg: &Message, time: u32);
    fn split(&self, msg: &Message) -> Vec<u32>;
    fn calculate_frequency(&self, key: u32) -> u32;
    fn get_raw_data(&self, msg: &Message, time: u32) -> Vec<f32>;

    fn get_name_msg(&self) -> Message {
        return Message::new().data(&self.get_name());
    }

    fn send_name(&self, channel: &dyn Channel, time: u32) {
        for key in self.split(&self.get_name_msg()) {
            channel.play(self.calculate_frequency(key), time);
        }
    }
}
