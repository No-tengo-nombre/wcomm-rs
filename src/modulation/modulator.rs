use crate::channels::Channel;
use crate::Message;
use crate::utils::math;

pub trait Modulator {
    fn get_name(&self) -> String;
    fn get_sampling_frequency(&self) -> u32;
    fn get_size(&self) -> u32;
    fn send_msg(&self, channel: &dyn Channel, msg: &Message, time: u32);
    fn get_raw_data(&self, msg: &Message, time: u32) -> Vec<f32>;

    fn split(&self, msg: &Message) -> Vec<u32> {
        let mut result = Vec::<u32>::new();
        for b in msg.group(math::log2(self.get_size())) {
            result.push(u32::from_str_radix(&b, 2).unwrap());
        }
        return result;
    }

    fn get_name_msg(&self) -> Message {
        return Message::new().data(&self.get_name());
    }

    fn send_name(&self, channel: &dyn Channel, time: u32) {
        self.send_msg(channel, &self.get_name_msg(), time);
    }
}
