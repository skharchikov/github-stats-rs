use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter};

pub struct Telemetry<T>
where
    T: SubscriberExt + Send + Sync + 'static,
{
    pub subscriber: T,
}

impl<T> Telemetry<T>
where
    T: SubscriberExt + Send + Sync + 'static,
{
    pub fn new<Sink>(
        name: String,
        env_filter: String,
        sink: Sink,
    ) -> Telemetry<impl SubscriberExt + Send + Sync + 'static>
    where
        Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
    {
        // JSON formatting layer
        let formatting_layer: BunyanFormattingLayer<Sink> = BunyanFormattingLayer::new(name, sink);

        // Filter layer
        let filter_layer =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

        Telemetry {
            subscriber: tracing_subscriber::Registry::default()
                .with(filter_layer)
                .with(formatting_layer),
        }
    }

    pub fn init(self) {
        LogTracer::init().expect("Failed to set logger");
        set_global_default(self.subscriber).expect("Failed to set subscriber");
    }
}
