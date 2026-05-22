pub mod field;
pub mod span;
pub mod subscriber;

#[cfg(test)]
mod tests {
    use crate::{
        field::Event,
        span::Span,
        subscriber::{Registry, StdoutSubscriber},
    };

    #[test]
    fn it_works() {
        let _ = Registry::new().layer(StdoutSubscriber);

        let _guard = Span::new("test").enter();
        Event::builder()
            .message("Hello world!".to_string())
            .timestamp(1)
            .build()
            .dispatch();
    }
}
