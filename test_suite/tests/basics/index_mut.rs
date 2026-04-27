use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From, Index, IndexMut, Eq)]
pub struct C(Vec<i32>);

#[allow(dead_code)]
#[derive(DerivingVia)]
#[deriving(From, Index, IndexMut)]
pub struct D<T>(Vec<T>);

#[test]
fn test() {
    let mut c = C(vec![1, 2, 4]);
    c[2] = 3;
    assert_eq!(c.0, vec![1, 2, 3]);
}
