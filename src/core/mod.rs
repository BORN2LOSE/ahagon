use gtk;
use gtk::prelude::*;
use gtk::{Builder, Button, Window};

mod dialog;

pub fn launch() {
    if gtk::init().is_err() {
        println!("Ahagon: failed to initialize GTK.");
        return;
    }

    /*
     *  First we get the file content ("name_of_glade_file.ui").
     *  And call the Builder call.
     *  Next, we define `window` var.
     */
    let ui = include_str!("../data/ui/app.ui");
    let builder = Builder::new_from_string(ui);
    let window: Window = builder.get_object("main_window").expect(
        "Couldn't get main_window",
    );

    let open_button = dialog::click_open;
    let about_button = dialog::click_about;
    /*
     *   Here we define `about` variable which implement
     *   "connect_clicked" method with `about_button` variable.
     */
    let about: Button = builder.get_object("about_btn").expect(
        "Couldn't get about_button",
    );

    let _setting: Button = builder.get_object("setup_btn").expect(
        "Couldn't get setup_button",
    );


    open_button(&builder, &window);
    // about_button and about = супер костыль.
    about.connect_clicked(move |x| about_button(x, &builder));

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    /*
     *   Recursively shows a widget (any child widgets)
     *   and start the gtk main loop.
     */
    window.show_all();
    gtk::main();
}
