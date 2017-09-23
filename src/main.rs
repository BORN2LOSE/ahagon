extern crate gtk;
extern crate gdk;

mod core;

#[cfg(target_os = "linux")]
fn main() {
    if cfg!(target_os = "linux") {
        core::gui::launch();
    }
}
