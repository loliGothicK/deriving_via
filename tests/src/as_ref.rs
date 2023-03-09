use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From)]
pub struct A(i32);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct B(A);

#[derive(DerivingVia)]
#[deriving(From, AsRef(via = i32))]
#[transitive(i32 -> A -> B -> C)]
pub struct C(B);

#[derive(DerivingVia)]
#[deriving(From, AsRef(via = T))]
pub struct D<T>(T);

#[test]
fn as_ref() {
    let c = C(B(A(1)));
    c.as_ref();
    let d = D(1);
    d.as_ref();
}
