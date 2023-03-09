use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(Into(via = i32))]
pub struct A(i32);

#[derive(DerivingVia)]
#[deriving(Into(via = i32))]
pub struct B(A);

#[derive(DerivingVia)]
#[deriving(Into(via = i32))]
pub struct C(B);

// not allowed
// see: https://doc.rust-lang.org/error_codes/E0210.html
#[derive(DerivingVia)]
pub struct D<T>(T);

#[test]
fn test() {
    let _: i32 = C(B(A(42))).into();
}
