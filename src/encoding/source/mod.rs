use crate::Message;
use std::collections::HashMap;

mod huffman;

pub use huffman::*;

/**
 * Type of source coding.
 */
pub trait SourceCoding {
    /**
     * Generate a code from the given message.
     */
    fn generate_code(msg: &Message) -> HashMap<u8, String>;

    /**
     * Encode the given message from this instance's code.
     */
    fn encode(&self, msg: &Message) -> Message;

    /**
     * Decode the given message from this instance's code.
     */
    fn decode(&self, msg: &Message, header_size: u32) -> Message;

    /**
     * Generate an empty mapping for chars.
     */
    fn empty_char_map() -> HashMap<u8, String> {
        let mut result = HashMap::<u8, String>::new();
        for key in 0..=255 {
            result.insert(key, "".to_string());
        }
        return result;
    }

    /**
     * Generate an empty frequency map.
     */
    fn empty_freq_map() -> HashMap<u8, u32> {
        let mut result = HashMap::<u8, u32>::new();
        for key in 0..=255 {
            result.insert(key, 0);
        }
        return result;
    }

    /**
     * Generate a frequency map of the chars in the given message.
     */
    fn generate_freq_map(msg: &Message) -> HashMap<u8, u32> {
        let mut result = Self::empty_freq_map();
        for c in msg.as_string().chars() {
            result.insert(c as u8, result.get(&(c as u8)).unwrap() + 1);
        }
        return result;
    }
}
