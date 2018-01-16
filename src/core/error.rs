#[derive(Clone, Debug)]
pub struct Error {
    name: String,
    message: String,
    // callTrace: Vec<T>,
}

impl Error {
    fn new(n: String, m: String) -> Error {
        Error {
            name: n,
            message: m,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn message(&self) -> &str {
        &self.message
    }
}
