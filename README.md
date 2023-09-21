# Chronoflake

Crate to generate unique 64-bit IDs in centralised or decentralised systems.

Based on the [Twitter Snowflake](https://blog.twitter.com/engineering/en_us/a/2010/announcing-snowflake) algorithm.

## Usage

```rust
use chronoflake::IdGenerator;

const MACHINE_ID: u16 = 49;
fn main() {
    // Create an ID generator using the default (Twitter) epoch
    let mut cf = IdGenerator::new(MACHINE_ID);

    // Generate a unique ID
    let id = cf.generate_id();

    // Futher processing...
}

