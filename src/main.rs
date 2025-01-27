use engine_lib::Engine;

#[macro_use] extern crate log;

const CODE: &str = r#"
var test = 58195815891891589015
"#;

fn main() {
    pretty_env_logger::formatted_builder()
        .target(pretty_env_logger::env_logger::Target::Stdout)
        .format_module_path(true)
        .filter_level(log::LevelFilter::Trace)
        .init();

    info!("Creating engine");

    let mut engine = Engine::default();

    match engine.exec(CODE) {
        Ok(status) => info!("Finished with status {status}"),
        Err(err) => error!("{err}")
    }
}
