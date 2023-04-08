use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(Default(via: T))]
pub struct D<T: Default>(T);

#[test]
fn test() {
    let _ = D::<i32>::default();
}
