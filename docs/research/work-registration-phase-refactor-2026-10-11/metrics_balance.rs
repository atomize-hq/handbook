//! Research-only telemetry balance smoke check; no native authority store.
#[path = "metrics.rs"]
mod metrics;
fn main() {
    metrics::begin();
    {
        let _outer = metrics::Span::enter("outer");
        let _inner = metrics::Span::enter("inner");
    }
    println!("{}", metrics::finish());
}
