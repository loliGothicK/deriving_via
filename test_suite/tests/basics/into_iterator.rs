use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(IntoIterator)]
struct MyVec(Vec<i32>);

#[test]
fn test() {
    assert_eq!(Some(5), MyVec(vec![5, 8]).into_iter().next());
}
