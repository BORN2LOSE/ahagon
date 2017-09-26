extern crate gtk;
// extern crate gdk;

mod core;

#[cfg(any(unix, target_os = "redox"))]
fn main() {
    core::gui::launch();
}
