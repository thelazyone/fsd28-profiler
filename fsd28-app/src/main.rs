fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    fsd28_app::run_app(); // Call the run_app function from your library crate
}
