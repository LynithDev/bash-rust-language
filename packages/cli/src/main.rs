use std::{path::PathBuf, str::FromStr};

use lang_engine::Engine;

#[macro_use] extern crate log;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::formatted_builder()
        .target(pretty_env_logger::env_logger::Target::Stdout)
        .format_module_path(true)
        .filter_level(log::LevelFilter::Trace)
        .init();

    info!("creating engine");

    let mut engine = Engine::default();

    match engine.exec_file(&PathBuf::from_str("./script.tsh")?) {
        Ok(status) => info!("finished with status {status}"),
        Err(err) => error!("{err}")
    };

    Ok(())
}
