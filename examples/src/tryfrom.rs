// ANCHOR: TryFrom
use std::convert::TryFrom;

use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(TryFrom(via: i64))]
pub struct Wrapper(i32);

impl std::convert::From<i64> for Wrapper {
    fn from(val: i64) -> Self {
        Wrapper(val as i32)
    }
}

#[allow(clippy::unnecessary_fallible_conversions)]
pub fn main() {
    let _ = Wrapper::try_from(42_i64);
}
// ANCHOR_END: TryFrom

#[test]
fn test_main() {
    main();
}
