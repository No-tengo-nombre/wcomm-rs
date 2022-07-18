use crate::Message;

pub trait Channel {
    fn send(&self, msg: Message, time: u32);
    fn play(&self, frequency: u32, time: u32);
    fn listen(&self, frequency: u32);
}
