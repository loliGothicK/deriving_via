use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From)]
pub struct A(i32);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct B(A);

#[derive(DerivingVia)]
#[deriving(From, Add(via = i32))]
#[transitive(i32 -> A -> B -> C)]
pub struct C(B);

#[test]
fn add() {
    let _ = C(B(A(1))) + C(B(A(1))) + C(B(A(1)));
}
