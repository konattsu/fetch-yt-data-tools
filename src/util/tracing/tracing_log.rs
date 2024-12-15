use tracing::{level_filters::LevelFilter, Subscriber};
use tracing_appender::rolling;
use tracing_subscriber::{
    fmt::{self, FormatEvent, FormatFields, FormattedFields},
    layer::SubscriberExt,
    registry::LookupSpan,
    util::SubscriberInitExt,
    EnvFilter, Layer,
};

/// 戻り値が生きている間ログの設定が有効になる
pub fn apply_tracing_settings(
    stdout_level: Option<LevelFilter>,
    file_level: Option<LevelFilter>,
) -> tracing_appender::non_blocking::WorkerGuard {
    let stdout_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .event_format(FormatterForStdout)
        .with_filter(filter_level(stdout_level));

    let file_appender = rolling::daily("./logs", "fetch-yt-data-tools.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = fmt::layer()
        .json()
        .with_writer(non_blocking)
        .with_filter(filter_level(file_level));

    tracing_subscriber::registry().with(stdout_layer).with(file_layer).init();
    guard
}

/// ログ出力のフィルターを指定
///
/// `None`: ログを出力しない
fn filter_level(level: Option<LevelFilter>) -> EnvFilter {
    match level.and_then(|lv| lv.into_level()) {
        Some(level) => EnvFilter::from(level.as_str()),
        None => {
            // details of constant, refs:
            // https://docs.rs/tracing-subscriber/0.3.18/src/tracing_subscriber/filter/env/directive.rs.html#125-139
            // https://docs.rs/tracing-core/0.1.32/src/tracing_core/metadata.rs.html#776-802
            const NO_OUTPUT: &str = "off";
            EnvFilter::new(NO_OUTPUT)
        }
    }
}

struct FormatterForStdout;

// refs:
// https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/trait.FormatEvent.html
impl<S, N> FormatEvent<S, N> for FormatterForStdout
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &fmt::FmtContext<'_, S, N>,
        mut writer: fmt::format::Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> std::fmt::Result {
        // Format values from the event's's metadata:
        let metadata = event.metadata();
        // write!(&mut writer, "{} {}: ", metadata.level(), metadata.target())?;

        write!(
            &mut writer,
            "{:<5} {} [{}:ln{}] ",
            metadata.level(),
            thread_id::get(),
            metadata.target(),
            metadata.line().unwrap_or_default()
        )?;

        if let Some(scope) = ctx.event_scope() {
            for span in scope.from_root() {
                write!(writer, "{}", span.name())?;

                // `FormattedFields` is a formatted representation of the span's
                // fields, which is stored in its extensions by the `fmt` layer's
                // `new_span` method. The fields will have been formatted
                // by the same field formatter that's provided to the event
                // formatter in the `FmtContext`.
                let ext = span.extensions();
                let fields =
                    &ext.get::<FormattedFields<N>>().expect("will never be `None`");

                // Skip formatting the fields if the span had no fields.
                if !fields.is_empty() {
                    write!(writer, "{{{}}}", fields)?;
                }
                write!(writer, ": ")?;
            }
        }

        // Write fields on the event
        ctx.field_format().format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}
