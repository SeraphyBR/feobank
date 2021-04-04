
mod session;
mod app;

use app::App;
use human_panic::setup_panic;

fn main() {
    setup_panic!();
    let mut app = App::new();
    app.run();
}
