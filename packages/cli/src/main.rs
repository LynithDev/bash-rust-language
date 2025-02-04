use lang_engine::Engine;

#[macro_use] extern crate log;

fn main() {
    pretty_env_logger::formatted_builder()
        .target(pretty_env_logger::env_logger::Target::Stdout)
        .format_module_path(true)
        .filter_level(log::LevelFilter::Trace)
        .init();

    debug!("initialized logger");

    let mut engine = Engine::create();


    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        error!("no input file provided");
        return;
    }

    let file_path = &args[1];
    let absolute_path = match std::fs::canonicalize(file_path) {
        Ok(path) => path,
        Err(err) => {
            error!("Could not get absolute path: {}", err);
            return;
        }
    };

    match engine.exec_file(&absolute_path) {
        Ok(status) => info!("finished with status {status}"),
        Err(err) => error!("{err}")
    };
}
