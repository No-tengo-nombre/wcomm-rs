use crate::Message;
use crate::channels::Channel;
use crate::modulation::Modulator;

pub struct MFSK {
    _size: u32,
    _base_frequency: u32,
    _delta_frequency: u32,
    _sampling_frequency: u32,
}

impl Modulator for MFSK {
    fn send_through_channel(&self, channel: &dyn Channel, msg: Message, time: u32) {}
}

impl MFSK {
    pub fn new(size: u32) -> MFSK {
        return MFSK {
            _size: size,
            _base_frequency: 300,
            _delta_frequency: 100,
            _sampling_frequency: 441000,
        };
    }

    pub fn size(mut self, new_size: u32) -> MFSK {
        self._size = new_size;
        return self;
    }

    pub fn base_frequency(mut self, new_base_freq: u32) -> MFSK {
        self._base_frequency = new_base_freq;
        return self;
    }

    pub fn delta_frequency(mut self, new_delta_freq: u32) -> MFSK {
        self._delta_frequency = new_delta_freq;
        return self;
    }

    pub fn sampling_frequency(mut self, new_samp_freq: u32) -> MFSK {
        self._sampling_frequency = new_samp_freq;
        return self;
    }
}
