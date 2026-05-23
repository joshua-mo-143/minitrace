use core::{event::Event, subscriber::SubscriberExt};

use jiff::tz::TimeZone;

pub struct StdoutSubscriber;

impl SubscriberExt for StdoutSubscriber {
    fn on_record_event(&self, event: &Event) {
        let message = event.message.as_ref().unwrap();
        let ts = jiff::Timestamp::from_second(event.timestamp as i64)
            .unwrap()
            .to_zoned(TimeZone::UTC)
            .to_string();

        println!("{ts} - {message}");
    }
}
