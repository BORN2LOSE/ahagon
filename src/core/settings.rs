use gtk;
use gtk::prelude::*;

pub fn create_setting_window() {
    let ui = include_str!("../resources/glade/setting.ui");
    let _build = gtk::Builder::new_from_string(ui);
    let win = gtk::Window::new(gtk::WindowType::Toplevel);

    win.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    win.show_all();
}
