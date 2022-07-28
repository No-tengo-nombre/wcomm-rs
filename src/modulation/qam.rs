use crate::channels::Channel;
use crate::modulation::Modulator;
use crate::Message;

pub struct MQAM {
    _size: u32,
    _base_frequency: u32,
    _sampling_frequency: u32,
}

impl Modulator for MQAM {
    fn get_name(&self) -> String {
        return format!("{}QAM", self._size);
    }

    fn get_sampling_frequency(&self) -> u32 {
        return self._sampling_frequency;
    }

    fn get_size(&self) -> u32 {
        return self._size;
    }

    fn send_msg(&self, channel: &dyn Channel, msg: &Message, time: u32) {
        // TODO
    }

    fn get_raw_data(&self, msg: &Message, time: u32) -> Vec<f32> {
        // TODO
        return Vec::new();
    }
}

impl MQAM {
    pub fn new(size: u32, carrier_freq: u32) -> MQAM {
        return MQAM {
            _size: size,
            _base_frequency: carrier_freq,
            _sampling_frequency: 44_100,
        };
    }

    pub fn size(mut self, new_size: u32) -> MQAM {
        self._size = new_size;
        return self;
    }

    pub fn base_frequency(mut self, new_base_freq: u32) -> MQAM {
        self._base_frequency = new_base_freq;
        return self;
    }

    pub fn sampling_frequency(mut self, new_samp_freq: u32) -> MQAM {
        self._sampling_frequency = new_samp_freq;
        return self;
    }

    pub fn calculate_amplitude(&self, key: u32) -> u32 {
        // TODO
        return 0;
    }
}
