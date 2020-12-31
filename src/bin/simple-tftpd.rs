// A simple TFTP server which serves `.`, the current working directory
use async_tftp::server::TftpServerBuilder;
use async_tftp::Error;
use futures_lite::future::block_on;

fn main() -> Result<(), Error> {
    logger_init();

    log::warn!("omg");

    block_on(async {
        let tftpd = TftpServerBuilder::with_dir_ro(".")?
            .bind("0.0.0.0:6969".parse().unwrap())
            // Workaround to handle cases where client is behind VPN
            .block_size_limit(1024)
            .build()
            .await?;

        log::info!("Listening on: {}", tftpd.listen_addr()?);
        tftpd.serve().await?;

        Ok(())
    })
}

// InfoLogger is a simple logger which prints info, warn & error log messages to stderr.
// It is based on samples from the log crate documentation.
//
// This is absolutely not a real log implementation and exists solely to avoid
// adding a specific log implementation as a dependency to this crate.
//
// For samples using fern see examples/{tftpd-dir.rs, tftpd-targz.rs}

use log::{Level, LevelFilter, Log, Metadata, Record};

static LOGGER: InfoLogger = InfoLogger;

pub fn logger_init() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .unwrap();
}

struct InfoLogger;

impl Log for InfoLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            eprintln!("{}: {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
