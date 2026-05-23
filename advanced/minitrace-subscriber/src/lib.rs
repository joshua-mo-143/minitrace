use core::{event::Event, subscriber::SubscriberExt};

pub struct StdoutSubscriber;

impl SubscriberExt for StdoutSubscriber {
    fn on_record_event(&self, event: &Event) {
        let message = event.message.as_ref().unwrap();
        let ts = event.timestamp;
        println!("{ts} - {message}");
    }
}
