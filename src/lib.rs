/**
 * Definitions for different types of channels which can transmit
 * data.
 */
pub mod channels;

/**
 * Types of encoding for messages.
 */
pub mod encoding;

/**
 * Types of digital modulation for messages.
 */
pub mod modulation;

/**
 * Different utilities for the creation and manipulation of messages.
 */
pub mod utils;

/**
 * Definitions for the creation of messages.
 */
pub mod message;

pub use message::Message;
