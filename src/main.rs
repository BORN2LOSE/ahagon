extern crate gtk;

mod core;

#[cfg(target_os = "linux")]
fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize Ahagon GTK.");
        return;
    }
    // See in `navigation.rs`.
    core::main_window();
    gtk::main();
}
