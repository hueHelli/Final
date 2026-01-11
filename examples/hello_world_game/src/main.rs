use engine_core::run;
use log::info;

fn main() {
    // Initialize env_logger with default settings.
    // This reads RUST_LOG env var for filtering
    env_logger::init();

    info!("Starting hello_world_game");

    // Hand control over to the engine.
    run();
}
