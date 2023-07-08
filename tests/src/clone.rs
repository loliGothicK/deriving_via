use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From)]
pub struct A(i32);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct B(A);

#[derive(DerivingVia)]
#[deriving(From, Clone(via: i32))]
#[transitive(i32 -> A -> B -> C)]
pub struct C(B);

#[derive(DerivingVia)]
#[deriving(From, Clone(via: T))]
pub struct D<T: Clone>(T);

#[test]
fn test() {
    let _: C = C(B(A(1))).clone();
    let _: D<i32> = D(1).clone();
}
