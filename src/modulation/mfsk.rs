use crate::channels::Channel;
use crate::modulation::Modulator;
use crate::utils::math;
use crate::Message;

pub struct MFSK {
    _size: u32,
    _base_frequency: u32,
    _delta_frequency: u32,
    _sampling_frequency: u32,
}

impl Modulator for MFSK {
    fn get_name(&self) -> String {
        return format!("{}FSK", self._size);
    }

    fn send_msg(&self, channel: &dyn Channel, msg: &Message, time: u32) {
        for key in self.split(msg) {
            channel.play(self.calculate_frequency(key), time);
        }
    }

    fn split(&self, message: &Message) -> Vec<u32> {
        let mut result = Vec::<u32>::new();
        for b in message.group(math::log2(self._size)) {
            result.push(isize::from_str_radix(&b, 2).unwrap() as u32);
        }
        return result;
    }

    fn calculate_frequency(&self, key: u32) -> u32 {
        return self._base_frequency + key * self._delta_frequency;
    }
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
