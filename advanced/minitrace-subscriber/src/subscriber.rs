use std::sync::{Arc, OnceLock};

use crate::field::Event;

pub(crate) static REGISTRY: OnceLock<Registry> = OnceLock::new();

pub struct Registry {
    pub(crate) subscribers: Arc<dyn SubscriberExt>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(NoopLayer),
        }
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

pub trait SubscriberExt: Send + Sync {
    fn on_record_event(&self, event: &Event);
}

impl<T> SubscriberExt for Arc<T>
where
    T: SubscriberExt + ?Sized,
{
    fn on_record_event(&self, event: &Event) {
        self.as_ref().on_record_event(event);
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
}

pub struct StdoutSubscriber;

impl SubscriberExt for StdoutSubscriber {
    fn on_record_event(&self, event: &Event) {
        let message = event.message.as_ref().unwrap();
        let ts = event.timestamp;
        println!("{ts} - {message}");
    }
}
