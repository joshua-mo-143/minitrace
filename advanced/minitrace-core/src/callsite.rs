use std::sync::{Mutex, OnceLock};

use crate::metadata::Metadata;

static CALLSITES: OnceLock<Mutex<Vec<&'static Callsite>>> = OnceLock::new();

pub struct Callsite {
    metadata: Metadata,
}

impl Callsite {
    pub fn register(metadata: Metadata) -> Self {
        Self { metadata }
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}
