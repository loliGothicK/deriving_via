use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From)]
pub struct A(i32);

#[derive(DerivingVia)]
#[deriving(Arithmetic(via = i32), AsRef(via = i32))]
pub struct B(A);

fn main() {
    let _ = B(A(42)) + B(A(42));
    let _ = B(A(42)) - B(A(42));
    let _ = B(A(42)) * B(A(42));
    let _ = B(A(42)) / B(A(42));

    let v = B(A(42));
    let _: &i32 = v.as_ref();
}
