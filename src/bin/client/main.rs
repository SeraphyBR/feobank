
mod session;
mod app;

use app::App;

fn main() {
    let mut app = App::new();
    app.run();
}
