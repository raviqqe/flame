use super::value::Value;

#[derive(Clone, Debug)]
pub struct Error {
    name: String,
    message: String,
    // callTrace: Vec<T>,
}

impl Error {
    fn new(n: &str, m: &str) -> Error {
        Error {
            name: String::from(n),
            message: String::from(m),
        }
    }

    fn value(m: &str) -> Error {
        Self::new("ValueError", m)
    }

    fn typ(v: Value, t: &str) -> Error {
        Self::new("TypeError", &format!("{} is not a {}", v, t))
    }

    fn not_boolean(v: Value) -> Error {
        Self::typ(v, "boolean")
    }

    fn not_dictionary(v: Value) -> Error {
        Self::typ(v, "dictionary")
    }

    fn not_function(v: Value) -> Error {
        Self::typ(v, "function")
    }

    fn not_list(v: Value) -> Error {
        Self::typ(v, "list")
    }

    fn not_nil(v: Value) -> Error {
        Self::typ(v, "nil")
    }

    fn not_number(v: Value) -> Error {
        Self::typ(v, "number")
    }

    fn not_string(v: Value) -> Error {
        Self::typ(v, "string")
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn message(&self) -> &str {
        &self.message
    }
}
