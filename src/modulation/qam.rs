use crate::channels::Channel;
use crate::modulation::DigitalModulator;
use crate::utils::math;
use crate::Message;

/**
 * M-ary Quadrature Amplitude Modulation.
 */
pub struct MQAM {
    _size: u32,
    _base_frequency: u32,
    _sampling_frequency: u32,
}

impl DigitalModulator for MQAM {
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
        let mut data = Vec::<f32>::new();
        let split_msg = self.split(msg);
        let samples_per_char = time * self.get_sampling_frequency() / 1000;

        for t in 0..(split_msg.len() as u32 * samples_per_char) {
            let key = split_msg[(t / samples_per_char) as usize];
            let cos_component = self.calculate_amplitude_cos(key)
                * (2_f32 * math::PI * ((self._base_frequency * t) as f32)
                    / self.get_sampling_frequency() as f32)
                    .cos();
            let sin_component = self.calculate_amplitude_sin(key)
                * (2_f32 * math::PI * ((self._base_frequency * t) as f32)
                    / self.get_sampling_frequency() as f32)
                    .sin();
            data.push(cos_component + sin_component);
        }
        return data;
    }
}

impl MQAM {
    /**
     * Create a new instance of `MQAM` with the given size and carrier frequency.
     */
    pub fn new(size: u32, carrier_freq: u32) -> MQAM {
        return MQAM {
            _size: size,
            _base_frequency: carrier_freq,
            _sampling_frequency: 44_100,
        };
    }

    /**
     * Set the size of the modulator.
     */
    pub fn size(mut self, new_size: u32) -> MQAM {
        self._size = new_size;
        return self;
    }

    /**
     * Set the base frequency/carrier frequency of the modulator.
     */
    pub fn base_frequency(mut self, new_base_freq: u32) -> MQAM {
        self._base_frequency = new_base_freq;
        return self;
    }

    /**
     * Set the sampling frequency of the modulator.
     */
    pub fn sampling_frequency(mut self, new_samp_freq: u32) -> MQAM {
        self._sampling_frequency = new_samp_freq;
        return self;
    }

    /**
     * Calculate the amplitude of the in-phase component for the given key.
     */
    pub fn calculate_amplitude_cos(&self, key: u32) -> f32 {
        let mut binary = format!("{:b}", key);

        // Preppend with zeros
        while (binary.len() as u32) < math::log2(self._size) {
            binary = format!("0{binary}");
        }
        let first = &binary[..binary.len() / 2];
        let first_num = u32::from_str_radix(&first, 2).unwrap();

        // Transform from gray code to indices
        let first_decoded = math::gray_to_num(first_num);
        return -(self._size as f32).sqrt() + (1 + 2 * first_decoded) as f32;
    }

    /**
     * Calculate the amplitude of the quadrature component for the given key.
     */
    pub fn calculate_amplitude_sin(&self, key: u32) -> f32 {
        let mut binary = format!("{:b}", key);

        // Preppend with zeros
        while (binary.len() as u32) < math::log2(self._size) {
            binary = format!("0{binary}");
        }
        let last = &binary[binary.len() / 2..];
        let last_num = u32::from_str_radix(&last, 2).unwrap();

        // Transform from gray code to indices
        let last_decoded = math::gray_to_num(last_num);
        return -(self._size as f32).sqrt() + (1 + 2 * last_decoded) as f32;
    }
}
