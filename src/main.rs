use engine_lib::Engine;

#[macro_use] extern crate log;

const CODE: &str = include_str!("../script.tsh");

fn main() {
    pretty_env_logger::formatted_builder()
        .target(pretty_env_logger::env_logger::Target::Stdout)
        .format_module_path(true)
        .filter_level(log::LevelFilter::Trace)
        .init();

    info!("creating engine");

    let mut engine = Engine::default();

    match engine.exec(CODE) {
        Ok(status) => info!("finished with status {status}"),
        Err(err) => error!("{err}")
    }
}
