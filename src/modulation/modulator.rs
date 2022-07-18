use crate::Message;
use crate::channels::Channel;

pub trait Modulator {
    fn send_through_channel(&self, channel: &dyn Channel, msg: Message, time: u32);
}
