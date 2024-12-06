use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(Iter)]
struct A(Vec<i32>);

#[derive(DerivingVia)]
#[deriving(Iter(via: Vec<i32>))]
struct B(A);

#[test]
fn test() {
    assert_eq!(Some(&5), A(vec![5, 8]).iter().next());
    assert_eq!(Some(&5), B(A(vec![5, 8])).iter().next());
}
