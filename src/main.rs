extern crate gtk;

mod core;
mod gui;

#[cfg(any(unix, target_os = "redox"))]
fn main() {
    gui::launch_gtk();
}
