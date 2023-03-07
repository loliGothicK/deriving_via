use deriving_via::DerivingVia;

#[derive(DerivingVia)]
pub struct A(i32);

#[derive(DerivingVia)]
pub struct B(A);

#[derive(DerivingVia)]
#[deriving(Display(via = i32))]
pub struct C(B);

fn main() {
    let c = C(B(A(42)));
    println!("{c}");
}
