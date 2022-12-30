#[cfg(test)]
// RUN cargo test --tests -- --nocapture
use serial_test::serial;

#[test]
#[serial]
fn test_sanity() {
    assert_eq!(1, 1);
}