extern crate gtk;

mod core;

#[cfg(target_os = "linux")]
fn main() {
    if cfg!(target_os = "linux") {
        core::gui::launch();
    }
}
