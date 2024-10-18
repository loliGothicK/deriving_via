use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From)]
#[debug("test({})")]
pub struct Test(i32);

#[test]
fn test() {
    assert_eq!(format!("{:?}", Test(1)), "test(1)".to_string());
}
