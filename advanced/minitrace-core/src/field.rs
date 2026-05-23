#[derive(Clone)]
pub struct Field {
    name: &'static str,
    pub value: String,
}

impl Field {
    fn new(name: &'static str, value: impl Into<String>) -> Self {
        Self {
            name,
            value: value.into(),
        }
    }
}
