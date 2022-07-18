use crate::Message;
use crate::channels::Channel;
use crate::modulation::Modulator;

pub struct MFSK {}

impl Modulator for MFSK {
    fn send_through_channel(&self, channel: &dyn Channel, msg: Message, time: u32) {}
}

impl MFSK {
    pub fn new() -> MFSK {
        return MFSK {};
    }
}
