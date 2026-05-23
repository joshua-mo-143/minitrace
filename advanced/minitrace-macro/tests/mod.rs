use minitrace_macro::span;

#[test]
fn disabled_span_does_not_evaluate_fields() {
    let _span = span!("test_span", expensive = "Hello world!");
}
