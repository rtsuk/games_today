use games_today::pages::GamesToday;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<GamesToday>();
}
