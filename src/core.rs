pub mod gui {
    use gtk;
    use gtk::prelude::*;

    use gtk::{Builder, Button, MessageDialog, Window};

    // Добавить окно для кнопки `About`.
    fn click_about(button: &Button, builder: &Builder) {
        let dialog: MessageDialog = builder.get_object("about_btn").expect(
            "Couldn't get dialog",
        );
        if let Some(window) = button.get_toplevel().and_then(
            |w| w.downcast::<Window>().ok(),
        )
        {
            dialog.set_transient_for(Some(&window));
        }

        // println!("Authors: {:?}", dialog.get_authors());
        dialog.show();
        dialog.run();
        dialog.hide();
    }

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
        let about: Button = builder.get_object("about_btn").expect(
            "Couldn't get about_button",
        );

        about.connect_clicked(move |x| click_about(x, &builder));

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        window.show_all();

        gtk::main();
    }
}
