use deriving_via::DerivingVia;
use std::str::FromStr;

#[derive(DerivingVia)]
#[deriving(FromStr)]
struct Id(String);

#[derive(DerivingVia)]
#[deriving(Display(via: String))]
struct UserId(Id);

impl FromStr for UserId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.starts_with("user-")
            .then_some(Self(s.parse().unwrap()))
            .ok_or(anyhow::anyhow!("validation error"))
    }
}

#[derive(DerivingVia)]
#[deriving(Display(via: String))]
struct ItemId(Id);

impl FromStr for ItemId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.starts_with("item-")
            .then_some(Self(s.parse().unwrap()))
            .ok_or(anyhow::anyhow!("validation error"))
    }
}

fn main() -> anyhow::Result<()> {
    let id: UserId = "user-mitama".parse()?;
    println!("{id}");
    let _: ItemId = "user-mitama".parse()?; // error

    Ok(())
}
