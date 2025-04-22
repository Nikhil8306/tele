#[derive(Debug)]
pub enum Error {
    FewArgs,
    TooManyArgs,
    MissingOption(String),
    UnknownOption(String),
    MissingValue(String)
}
