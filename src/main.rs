extern crate gtk;

mod core;

#[cfg(target_os = "linux")]
fn main() {
    core::gui::launch();
}
