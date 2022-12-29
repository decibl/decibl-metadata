#[cfg(test)]

// RUN cargo test --tests

use decibl_metadata::engine::analyticsdb;
use decibl_metadata::engine::analyticsdb::*;
use decibl_metadata::engine::audio_metadata::*;
use decibl_metadata::engine::config::*;
use decibl_metadata::engine::models::*;

#[test]
fn brah() {
    assert_eq!(2 + 2, 4);
}