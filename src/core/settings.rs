// use gtk;
// use gtk::prelude::*;

// pub fn create_setting_window(application: &gtk::Application) {
//     let glade_src = include_str!("../resources/glade/setting.ui");
//     let builder = gtk::Builder::new_from_string(ui);
//     let setting_window = gtk::Window::new(gtk::WindowType::Toplevel);

//     setting_window.set_application(application);
//     setting_window.connect_delete_event(clone!(main_window => move |_, _| {
//             main_window.destroy();
//             Inhibit(false)
//     }));

//     setting_window.show_all();
// }

// pub fn launch_setting(arg: Type) -> RetType {
//     unimplemented!();
// }
