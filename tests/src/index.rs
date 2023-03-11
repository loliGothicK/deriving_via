use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(Index, FromIterator(via: i32))]
pub struct A(Vec<i32>);

#[derive(DerivingVia)]
#[deriving(Index(via: Vec<i32>), From)]
pub struct B(A);

#[derive(DerivingVia)]
#[deriving(Index, FromIterator(via: T))]
pub struct C<T>(Vec<T>);

#[derive(DerivingVia)]
#[deriving(Index(via: Vec<T>), From)]
pub struct D<T>(C<T>);

#[test]
fn test() {
    let a: A = [0, 1, 2, 3].into_iter().collect();
    let _ = a[0];
    let b: B = a.into();
    let _ = b[0];
    let c: C<i32> = [0, 1, 2, 3].into_iter().collect();
    let _ = c[0];
    let d: D<i32> = c.into();
    let _ = d[0];
}
