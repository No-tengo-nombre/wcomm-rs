use crate::encoding::source::SourceCoding;
use crate::Message;
use std::collections::HashMap;

pub struct HuffmanCoding {
    _code: HashMap<u8, String>,
}

impl SourceCoding for HuffmanCoding {
    fn generate_code(msg: &Message) -> HashMap<u8, String> {
        // TODO: Implement this
        return HuffmanCoding::empty_char_map();
    }

    fn encode(&self, msg: &Message) -> Message {
        // TODO: Implement this
        return Message::new();
    }

    fn decode(&self, msg: &Message) -> Message {
        // TODO: Implement this
        return Message::new();
    }
}

impl HuffmanCoding {
    pub fn new() -> HuffmanCoding {
        return HuffmanCoding {
            _code: HuffmanCoding::empty_char_map(),
        };
    }

    pub fn code(mut self, new_code: HashMap<u8, String>) -> HuffmanCoding {
        self._code = new_code;
        return self;
    }
}
