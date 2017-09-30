extern crate gtk;

mod core;

#[cfg(any(unix, target_os = "redox"))]
fn main() {
    core::launch();
}
