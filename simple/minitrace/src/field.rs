use crate::subscriber::REGISTRY;

pub struct Event {
    pub(crate) timestamp: u64,
    pub(crate) message: Option<String>,
}

impl Event {
    pub fn builder() -> EventBuilder {
        EventBuilder::new()
    }

    pub fn dispatch(self) {
        if let Some(reg) = REGISTRY.get() {
            reg.subscribers.on_record_event(&self);
        }
    }
}

pub struct EventBuilder {
    timestamp: Option<u64>,
    message: Option<String>,
}

impl EventBuilder {
    pub fn new() -> Self {
        Self {
            timestamp: None,
            message: None,
        }
    }

    pub fn timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn timestamp_opt(mut self, timestamp: Option<u64>) -> Self {
        self.timestamp = timestamp;
        self
    }

    pub fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    pub fn message_opt(mut self, message: Option<String>) -> Self {
        self.message = message;
        self
    }

    pub fn build(self) -> Event {
        let Self { timestamp, message } = self;

        Event {
            timestamp: timestamp.unwrap(),
            message,
        }
    }
}
