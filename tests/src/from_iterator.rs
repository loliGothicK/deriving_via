use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(FromIterator(via: i32))]
pub struct A(Vec<i32>);

#[derive(DerivingVia)]
#[deriving(FromIterator(via: T))]
pub struct B<T>(Vec<T>);

#[test]
fn test() {
    let _: A = [0, 1, 2, 3].into_iter().collect();
    let _: B<i32> = [0, 1, 2, 3].into_iter().collect();
}
