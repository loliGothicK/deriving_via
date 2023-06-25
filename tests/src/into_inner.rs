use deriving_via::DerivingVia;

#[derive(DerivingVia, Copy, Clone)]
#[deriving(From, IntoInner)]
pub struct A(i32);

#[derive(DerivingVia, Copy, Clone)]
#[deriving(From)]
pub struct B(A);

#[derive(DerivingVia, Copy, Clone)]
#[deriving(From, IntoInner(via: i32))]
pub struct C(B);

#[test]
fn test() {
    let c = C(B(A(1)));
    let _: i32 = c.into_inner();
    let _: i32 = c.into_inner();
}
