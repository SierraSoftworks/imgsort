use std::fmt::Display;

pub enum Value<'a> {
    Borrowed(&'a str),
    Owned(String),
}

impl<'a> Value<'a> {
    pub fn as_str(&self) -> &str {
        match self {
            Value::Borrowed(s) => s,
            Value::Owned(s) => s,
        }
    }
}

impl<'a> Default for Value<'a> {
    fn default() -> Self {
        Value::Borrowed("")
    }
}

impl<'a> Display for Value<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for Value<'_> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<'a> From<&'a str> for Value<'a> {
    fn from(s: &'a str) -> Self {
        Value::Borrowed(s)
    }
}

impl<'a> From<String> for Value<'a> {
    fn from(s: String) -> Self {
        Value::Owned(s)
    }
}

impl<'a> From<&'a String> for Value<'a> {
    fn from(s: &'a String) -> Self {
        Value::Borrowed(s.as_ref())
    }
}
