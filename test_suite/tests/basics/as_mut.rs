use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From, AsMut, Eq)]
pub struct C(Vec<i32>);

#[derive(DerivingVia)]
#[deriving(From, AsMut(via: Vec<T>))]
pub struct D<T>(Vec<T>);

#[test]
fn test() {
    let mut c = C(vec![1, 2, 4]);
    let _: &mut [i32] = c.as_mut();
}
