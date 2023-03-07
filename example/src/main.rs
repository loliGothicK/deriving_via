use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From)]
pub struct Inner(i32);

#[derive(DerivingVia)]
#[deriving(FromStr(via = i32))]
pub struct Outer(Inner);

fn main() {
    let _: Outer = "42".parse().unwrap();
}
