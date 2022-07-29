use crate::Message;

/**
 * Channel caracterizing a given medium through which signals spread.
 */
pub trait Channel {
    /**
     * Send a given message through this channel.
     */
    fn send(&self, msg: &Message, time: u32);

    /**
     * Plays a tone of the given frecuency.
     */
    fn play(&self, frequency: u32, time: u32);

    /**
     * Listens for a message in the given frecuency.
     */
    fn listen(&self, frequency: u32);
}
