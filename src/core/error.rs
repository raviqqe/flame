use super::value::Value;

#[derive(Clone, Debug)]
pub struct Error {
    name: String,
    message: String,
    // callTrace: Vec<T>,
}

impl Error {
    pub fn new(n: &str, m: &str) -> Error {
        Error {
            name: String::from(n),
            message: String::from(m),
        }
    }

    pub fn argument(m: &str) -> Error {
        Self::new("ArgumentError", m)
    }

    pub fn value(m: &str) -> Error {
        Self::new("ValueError", m)
    }

    pub fn typ(v: Value, t: &str) -> Error {
        Self::new("TypeError", &format!("{} is not a {}", v, t))
    }

    pub fn not_boolean(v: Value) -> Error {
        Self::typ(v, "boolean")
    }

    pub fn not_dictionary(v: Value) -> Error {
        Self::typ(v, "dictionary")
    }

    pub fn not_function(v: Value) -> Error {
        Self::typ(v, "function")
    }

    pub fn not_list(v: Value) -> Error {
        Self::typ(v, "list")
    }

    pub fn not_nil(v: Value) -> Error {
        Self::typ(v, "nil")
    }

    pub fn not_number(v: Value) -> Error {
        Self::typ(v, "number")
    }

    pub fn not_string(v: Value) -> Error {
        Self::typ(v, "string")
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
