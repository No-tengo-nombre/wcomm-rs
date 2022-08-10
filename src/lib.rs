use std::fmt;

mod channels;
mod encoding;
mod modulation;
pub mod utils;

pub use channels::*;
pub use encoding::*;
pub use modulation::*;

/**
 * Container of binary data, which also includes a header. This header
 * is a special collection of data which isn't used in things like
 * encoding, as it is meant to be a lightweight message with
 * information about the contained data.
 */
pub struct Message {
    _data: String,
    _header: String,
    _total: String,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.as_string());
    }
}

impl Message {
    /**
     * Create an empty message.
     */
    pub fn new() -> Message {
        return Message {
            _data: "".to_string(),
            _header: "".to_string(),
            _total: "".to_string(),
        };
    }

    /**
     * Create a message from some given data and a header as strings.
     * These are transformed into binary and then stored.
     */
    pub fn from_string(data: &str, header: &str) -> Message {
        return Message {
            _data: string_to_binary(&data),
            _header: string_to_binary(&header),
            _total: string_to_binary(&header) + &string_to_binary(&data),
        };
    }

    /**
     * Create a message from some given data and header binaries.
     */
    pub fn from_binary(data: &str, header: &str) -> Message {
        return Message {
            _data: data.to_string(),
            _header: header.to_string(),
            _total: header.to_string() + &data.to_string(),
        };
    }

    /**
     * Set the data of the message.
     */
    pub fn data(mut self, data: &str) -> Message {
        self._data = string_to_binary(&data);
        self._total = self._header.clone() + &self._data;
        return self;
    }

    /**
     * Set the data of the message as a binary.
     */
    pub fn data_from_binary(mut self, data: &str) -> Message {
        self._data = data.to_string();
        self._total = self._header.clone() + &self._data;
        return self;
    }

    /**
     * Set the header of the message.
     */
    pub fn header(mut self, header: &str) -> Message {
        self._header = string_to_binary(&header.to_string());
        self._total = self._header.clone() + &self._data;
        return self;
    }

    /**
     * Set the header of the message as a binary.
     */
    pub fn header_from_binary(mut self, header: &str) -> Message {
        self._header = header.to_string();
        self._total = self._header.clone() + &self._data;
        return self;
    }

    /**
     * Get the data of the message as binary.
     */
    pub fn get_data(&self) -> &str {
        return &self._data;
    }

    /**
     * Get the header of the message as binary.
     */
    pub fn get_header(&self) -> &str {
        return &self._header;
    }

    /**
     * Get the total amount of bits in the message.
     */
    pub fn bit_size(&self) -> u32 {
        return (self._total.len()) as u32;
    }

    /**
     * Get the amount of bits in the data part of the message.
     */
    pub fn data_size(&self) -> u32 {
        return self._data.len() as u32;
    }

    /**
     * Get the amount of bits in the header part of the message.
     */
    pub fn header_size(&self) -> u32 {
        return self._header.len() as u32;
    }

    /**
     * Group a string in packets of a given size.
     */
    pub fn group_string(message: &String, num: u32) -> Vec<String> {
        let mut result = Vec::<String>::new();
        let mut temp = message.clone();
        while temp != "" {
            result.push(temp[..(num as usize)].to_string());
            temp = temp[(num as usize)..].to_string();
        }
        return result;
    }

    /**
     * Group the message in packets of the given size.
     */
    pub fn group(&self, num: u32) -> Vec<String> {
        return Message::group_string(&self._total, num);
    }

    /**
     * Group the data in packets of the given size.
     */
    pub fn group_data(&self, num: u32) -> Vec<String> {
        return Message::group_string(&self._data, num);
    }

    /**
     * Group the header in packets of the given size.
     */
    pub fn group_header(&self, num: u32) -> Vec<String> {
        return Message::group_string(&self._header, num);
    }

    /**
     * Get the message as an array of 8-bit integers.
     */
    pub fn as_u8_array(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::new();
        for c in self.group(8) {
            result.push(u8::from_str_radix(&c, 2).unwrap());
        }
        return result;
    }

    /**
     * Get the data as an array of 8-bit integers.
     */
    pub fn data_as_u8_array(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::new();
        for c in self.group_data(8) {
            result.push(u8::from_str_radix(&c, 2).unwrap());
        }
        return result;
    }

    /**
     * Get the header as an array of 8-bit integers.
     */
    pub fn header_as_u8_array(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::new();
        for c in self.group_header(8) {
            result.push(u8::from_str_radix(&c, 2).unwrap());
        }
        return result;
    }

    /**
     * Get the message as a binary string.
     */
    pub fn as_binary(&self) -> &str {
        return &self._total;
    }

    /**
     * Get the data as a binary string.
     */
    pub fn data_as_binary(&self) -> &str {
        return &self._data;
    }

    /**
     * Get the header as a binary string.
     */
    pub fn header_as_binary(&self) -> &str {
        return &self._header;
    }

    /**
     * Get the message as a decoded string.
     */
    pub fn as_string(&self) -> String {
        let mut result = "".to_string();
        for c in self.as_u8_array() {
            result += &(c as char).to_string();
        }
        return result;
    }

    /**
     * Get the data as a decoded string.
     */
    pub fn data_as_string(&self) -> String {
        let mut result = "".to_string();
        for c in self.data_as_u8_array() {
            result += &(c as char).to_string();
        }
        return result;
    }

    /**
     * Get the header as a decoded string.
     */
    pub fn header_as_string(&self) -> String {
        let mut result = "".to_string();
        for c in self.header_as_u8_array() {
            result += &(c as char).to_string();
        }
        return result;
    }
}

/**
 * Transform a string into a binary string.
 */
pub fn string_to_binary(data: &str) -> String {
    let mut result = "".to_string();
    for c in data.bytes() {
        result += &format!("{c:0>8b}");
    }
    return result;
}
