use std::sync::{Mutex, OnceLock};

static SPAN_REGISTRY: OnceLock<Mutex<Vec<Span>>> = OnceLock::new();

#[derive(Clone)]
pub struct Span {
    name: String,
}

impl Span {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn enter(self) -> Spanguard {
        let name = self.name.clone();
        let registry = SPAN_REGISTRY.get_or_init(|| Mutex::new(Vec::new()));
        registry.lock().unwrap().push(self);

        Spanguard { name }
    }

    pub fn in_scope<F, T>(&self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        let _guard = self.clone().enter();

        f()
    }
}

pub struct Spanguard {
    name: String,
}

impl Spanguard {
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Drop for Spanguard {
    fn drop(&mut self) {
        let registry = SPAN_REGISTRY.get_or_init(|| Mutex::new(Vec::new()));
        registry.lock().unwrap().pop();
    }
}

#[cfg(test)]
mod tests {
    use crate::span::Span;

    #[test]
    fn span() {
        let _span = Span::new("test");
        let _guard = _span.enter();
    }
}
