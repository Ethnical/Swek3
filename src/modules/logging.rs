

use env_logger::{fmt::Color, Env};
pub use log::{error, info};
use std::io::Write;

pub fn configure_logging(
    logging_destination: Option<Box<dyn Write + Send + 'static>>,
) {
    let mut builder = env_logger::Builder::from_env(Env::default().default_filter_or("info"));
    let mut builder = builder.format(move |buf, record| {
    let mut level_style = buf.style();
    level_style.set_color(Color::Green).set_bold(true);
    let mut target_style = buf.style();
    target_style.set_color(Color::Blue);

        let mut blockchain_style = buf.style();
        blockchain_style.set_color(Color::Yellow);
        writeln!(
                buf,
                "[{} {} {}] {}",
                buf.timestamp(),
                level_style.value(record.level()),
                target_style.value(record.target()),
                record.args()
            )

    });
    if logging_destination.is_some() {
        builder = builder.target(env_logger::Target::Pipe(logging_destination.unwrap()))
    }
    builder.init();
}
