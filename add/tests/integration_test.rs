use add_one;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, add_one::add_two(2));
}
