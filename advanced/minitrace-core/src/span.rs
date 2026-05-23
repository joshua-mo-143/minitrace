use std::sync::{Mutex, OnceLock};

static SPAN_REGISTRY: OnceLock<Mutex<Vec<Span>>> = OnceLock::new();

#[derive(Clone)]
pub struct Span {
    name: String,
    enabled: bool,
}

impl Span {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            enabled: true,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn disabled() -> Self {
        Self {
            name: String::new(),
            enabled: false,
        }
    }

    pub fn enter<'a>(self) -> Spanguard<'a> {
        if !self.enabled {
            return Spanguard { enabled: &false };
        }

        let registry = SPAN_REGISTRY.get_or_init(|| Mutex::new(Vec::new()));
        registry.lock().unwrap().push(self);

        Spanguard { enabled: &true }
    }

    pub fn in_scope<F, T>(&self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        let _guard = self.clone().enter();

        f()
    }
}

pub struct Spanguard<'a> {
    enabled: &'a bool,
}

impl<'a> Spanguard<'a> {
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }
}

impl<'a> Drop for Spanguard<'a> {
    fn drop(&mut self) {
        if *self.enabled() {
            let registry = SPAN_REGISTRY.get_or_init(|| Mutex::new(Vec::new()));
            registry.lock().unwrap().pop();
        }
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
