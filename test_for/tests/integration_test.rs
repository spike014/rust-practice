use test_for;
mod common;

#[test]
fn it_adds_two_intergation() {
    common::setup();
    assert_eq!(4, test_for::add_two(2));
}
