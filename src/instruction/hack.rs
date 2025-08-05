pub trait Hackable {
    fn to_hack(self: Self) -> Result<String, String>;
}

