//! Generate unique IDs for a variety of purposes.
//!
//! Based on the Twitter Snowflake algorithm for generating unique IDs for messages
//!
//! # Usage
//!
//! ```rust
//! use chronoflake::IdGenerator;
//!
//! const PROJECT_EPOCH: u64 = 1488432924251;
//! fn main() {
//!     let mut cf = IdGenerator::new(14)
//!         .with_epoch(PROJECT_EPOCH);
//!
//!     let id = cf.generate_id();
//!     println!("ID: {id}"); // 1704967240656416804
//! }
//! ```
use chrono::Utc;

/// Default time epoch to use (Twitter Epoch)
pub const DEFAULT_EPOCH: u64 = 1288834974657;

/// Unique ID generator
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct IdGenerator {
    /// Time Epoch to use (in milliseconds)
    pub epoch: u64,

    /// Machine or Shard ID
    pub shard_id: u16,

    /// Sequence number to track sequential ID generated within a timeframe
    pub sequence: u16,

    /// Timeframe in which sequences can increase
    pub timestamp: u64,
}

impl IdGenerator {
    /// Create a new Chronoflake ID Generator
    ///
    /// ```rust
    /// use chronoflake::IdGenerator;
    ///
    /// let mut cf = IdGenerator::new(16);
    /// ```
    pub fn new(shard_id: u16) -> Self {
        Self {
            epoch: DEFAULT_EPOCH,
            shard_id,
            sequence: 0,
            timestamp: Utc::now().timestamp_millis() as u64,
        }
    }

    /// Set the epoch for the generator
    ///
    /// ```rust
    /// use chronoflake::IdGenerator;
    ///
    /// let mut cf = IdGenerator::new(16).with_epoch(1488432924251);
    /// ```
    pub fn with_epoch(mut self, epoch: u64) -> Self {
        self.epoch = epoch;
        self
    }

    /// Generate a unique ID
    ///
    /// ```rust
    /// use chronoflake::IdGenerator;
    ///
    /// let mut cf = IdGenerator::new(16).with_epoch(1488432924251);
    /// let id = cf.generate_id();
    /// println!("ID: {id}"); // 1704967240656416804
    /// ```
    pub fn generate_id(&mut self) -> u64 {
        let now = Utc::now().timestamp_millis() as u64;
        if now > self.timestamp + 1 {
            self.timestamp = now;
            self.sequence = 0;
        }

        let ts = now - self.epoch;
        let id = ((ts & 0x1FFFFFFFFFF) << 22)
            | ((self.shard_id as u64 & 0x3FF) << 12)
            | (self.sequence as u64 & 0xFFF);

        self.sequence += 1;

        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mass_unique() {
        let mut cf = IdGenerator::new(49);

        let mut prev_id: u64 = 0;
        for _ in 0..50_000_000 {
            let id = cf.generate_id();
            assert!(prev_id != id);
            prev_id = id;
        }
    }
}
