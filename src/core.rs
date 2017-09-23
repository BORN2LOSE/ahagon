pub mod gui {
    use gtk;
    use gtk::prelude::*;

    #[allow(unused_imports)]
    use gtk::{Builder, Button, MessageDialog, Window};

    pub fn launch() {
        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
            return;
        }

        let ui = include_str!("app.ui");
        let builder = Builder::new_from_string(ui);

        let window: Window = builder.get_object("main_window").expect(
            "Couldn't get main_window",
        );
        let _open: Button = builder.get_object("open_btn").expect(
            "Couldn't get open_button",
        );
        let _setting: Button = builder.get_object("setup_btn").expect(
            "Couldn't get setup_button",
        );
        let _about: Button = builder.get_object("about_btn").expect(
            "Couldn't get about_button",
        );

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        window.show_all();

        gtk::main();
    }
}
