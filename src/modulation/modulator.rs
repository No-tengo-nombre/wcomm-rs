use crate::channels::Channel;
use crate::utils::math;
use crate::Message;

/**
 * Any type of digital modulator.
 */
pub trait DigitalModulator {
    /**
     * Get the ASCII name of the modulator (e.g, "256FSK").
     */
    fn get_name(&self) -> String;

    /**
     * Get the sampling frequency.
     */
    fn get_sampling_frequency(&self) -> u32;

    /**
     * Get the size of the modulator.
     * e.g: For 256-FSK, the size is 256.
     */
    fn get_size(&self) -> u32;

    /**
     * Send a message through the given channel.
     */
    fn send_msg(&self, channel: &dyn Channel, msg: &Message, time: u32);

    /**
     * Gets the raw data of a message, corresponding to the amplitude values
     * over time.
     */
    fn get_raw_data(&self, msg: &Message, time: u32) -> Vec<f32>;

    /**
     * Split the message using the size of the modulator.
     */
    fn split(&self, msg: &Message) -> Vec<u32> {
        let mut result = Vec::<u32>::new();
        for b in msg.group(math::log2(self.get_size())) {
            result.push(u32::from_str_radix(&b, 2).unwrap());
        }
        return result;
    }

    /**
     * Get the name of the modulator as an instance of `Message`.
     */
    fn get_name_msg(&self) -> Message {
        return Message::new().data(&self.get_name());
    }

    /**
     * Send the name of the modulator through the given channnel.
     */
    fn send_name(&self, channel: &dyn Channel, time: u32) {
        self.send_msg(channel, &self.get_name_msg(), time);
    }
}
