extern crate uptime_lib;

#[test]
fn test_uptime_get() {
    assert!(uptime_lib::get().is_ok());
}
