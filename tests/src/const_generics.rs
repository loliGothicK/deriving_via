use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From, Index, Iter)]
pub struct Test<const N: usize>([i32; N]);

#[test]
fn test() {
    let test: Test<4> = [1, 2, 3, 4].into();
    assert_eq!(test[0], 1);

    assert_eq!(test.iter().as_slice(), &[1, 2, 3, 4]);
}
