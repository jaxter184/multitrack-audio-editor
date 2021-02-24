
use tuix::*;

pub mod ui;
pub use ui::*;


fn main() {
    let app = Application::new(|window_description, state, root| {

        state.add_stylesheet("src/ui/block_style.css").expect("Failed to load stylesheet");

        App::new().build(state, root, |builder| builder);

        window_description.with_title("Multitrack Audio Editor")
    });

    app.run();
}