use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From)]
pub struct A(i32);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct B(A);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct C(B);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct D<T>(T);

#[test]
fn test() {
    let a = A(1);
    let b: B = From::from(a);
    let c: C = From::from(b);

    let _: D<C> = From::from(c);
}
