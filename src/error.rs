pub enum Error {
    FewArgs,
    TooManyArgs,
    MissingOption(String),
    UnknownOption(String),
    MissingValue(String)
}

// impl Error {
//     fn new(msg: String) -> Self {
//         Self { msg }
//     }
// }