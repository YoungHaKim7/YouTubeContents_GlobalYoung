extern crate test_lib;
use test_lib::to_test;

#[test]
fn test_to_test() {
    assert_eq!(to_test(true), true);
}
// fn it_adds_two() {
//     assert_eq!(4, adder::add_two(2));
// }
