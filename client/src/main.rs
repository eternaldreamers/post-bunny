use gpui::{App, Application};

use post_bunny_panel::bootstrap::{self, Assets};
use post_bunny_panel::storage;

fn main() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        bootstrap::register_keybindings(cx);
        let language = storage::load_language();
        bootstrap::open_window(cx, language);
    });
}
