use std::sync::{Arc, OnceLock};

use crate::{event::Event, metadata::Metadata};

pub trait SubscriberExt: Send + Sync {
    fn on_record_event(&self, event: &Event);
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
}

impl<T> SubscriberExt for Arc<T>
where
    T: SubscriberExt + ?Sized,
{
    fn on_record_event(&self, event: &Event) {
        self.as_ref().on_record_event(event);
    }

    fn enabled(&self, metadata: &Metadata) -> bool {
        self.as_ref().enabled(metadata)
    }
}

pub static REGISTRY: OnceLock<Registry> = OnceLock::new();

pub struct Registry {
    pub(crate) subscribers: Arc<dyn SubscriberExt>,
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(NoopLayer),
        }
    }

    pub fn enabled(&self, metadata: &Metadata) -> bool {
        self.subscribers.enabled(&metadata)
    }
}

impl Registry {
    pub fn layer<T1>(self, layer: T1) -> Registry
    where
        T1: SubscriberExt + 'static,
    {
        Registry {
            subscribers: Arc::new(LayeredEvent::new(self.subscribers, layer)),
        }
    }

    pub fn init(self) {
        assert!(
            REGISTRY.set(self).is_ok(),
            "global subscriber registry already initialized"
        );
    }
}

struct NoopLayer;

impl SubscriberExt for NoopLayer {
    fn on_record_event(&self, _event: &Event) {}
}

struct LayeredEvent<S, T>
where
    S: SubscriberExt,
    T: SubscriberExt,
{
    outer: S,
    inner: T,
}

impl<S, T> LayeredEvent<S, T>
where
    S: SubscriberExt,
    T: SubscriberExt,
{
    pub fn new(outer: S, inner: T) -> Self {
        Self { outer, inner }
    }
}

/// Do nothing here.
impl<S, T> SubscriberExt for LayeredEvent<S, T>
where
    S: SubscriberExt,
    T: SubscriberExt,
{
    fn on_record_event(&self, event: &Event) {
        self.inner.on_record_event(event);
        self.outer.on_record_event(event);
    }

    fn enabled(&self, metadata: &Metadata) -> bool {
        self.inner.enabled(metadata) || self.outer.enabled(metadata)
    }
}
