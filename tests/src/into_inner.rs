use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From, IntoInner)]
pub struct A(i32);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct B(A);

#[derive(DerivingVia)]
#[deriving(From, IntoInner(via: i32))]
pub struct C(B);

#[test]
fn test() {
    let a = A(1);
    let _: i32 = a.into_inner();
    let c = C(B(A(1)));
    let _: i32 = c.into_inner();
}
