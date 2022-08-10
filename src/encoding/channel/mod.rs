mod hamming;

use std::{error, fmt};

pub use hamming::*;

use crate::Message;

#[derive(Debug)]
pub struct ChannelError {
    _bit_num: u32,
}

impl ChannelError {
    pub fn new(num: u32) -> ChannelError {
        return ChannelError { _bit_num: num };
    }
}

impl fmt::Display for ChannelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Encountered {} wrong bits.", self._bit_num);
    }
}

impl error::Error for ChannelError {}

pub trait ChannelCoding {
    fn encode(msg: &Message) -> Message;
    fn decode(msg: &Message) -> Result<Message, ChannelError>;
}
