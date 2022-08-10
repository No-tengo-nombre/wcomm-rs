use crate::ChannelCoding;

struct HammingCoding {
    _total: u32,
    _data: u32,
}

impl HammingCoding {
    pub fn new(total: u32, data: u32) -> HammingCoding {
        return HammingCoding {
            _total: total,
            _data: data,
        };
    }

    pub fn bit_num(&self) -> u32 {
        return self._total - self._data;
    }
}

impl Default for HammingCoding {
    fn default() -> HammingCoding {
        return HammingCoding {
            _total: 8,
            _data: 4,
        };
    }
}

impl ChannelCoding for HammingCoding {
    fn encode(msg: &crate::Message) -> crate::Message {
        todo!();
    }

    fn decode(msg: &crate::Message) -> Result<crate::Message, crate::ChannelError> {
        todo!();
    }
}
