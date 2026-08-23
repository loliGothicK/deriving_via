#![no_std]
use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(
    Display, Into, From, Eq, Ord, Hash, Arithmetic, AddAssign, MulAssign, Default, Copy, Debug,
    IntoInner
)]
pub struct NoStdStruct(i32);

#[derive(DerivingVia)]
#[deriving(TryFrom)]
pub struct NoStdTry(i32);

#[derive(DerivingVia)]
#[deriving(AsRef, AsMut, Index, IndexMut)]
pub struct NoStdRefMut([i32; 1]);

#[derive(DerivingVia)]
#[deriving(IntoIterator, Iter)]
pub struct NoStdIter([i32; 1]);

#[test]
fn test_no_std_derives() {
    let s1 = NoStdStruct(42);
    let s2 = NoStdStruct(24);
    let _s3 = s1 + s2;
    let _t = NoStdTry(42);
    let mut r = NoStdRefMut([42]);
    let _: &[i32] = r.as_ref();
    let _: &mut [i32] = r.as_mut();
    let _ = r[0];
    r[0] = 24;
    let iter = NoStdIter([42]);
    for _ in iter.iter() {}
    for _ in iter {}
}
