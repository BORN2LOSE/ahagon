use gtk;
use gtk::prelude::*;
use gtk::{Builder, Button, Window};

use core::dialog;
use core::settings;

pub fn launch_gtk() {
    if gtk::init().is_err() {
        println!("Ahagon: failed to initialize GTK.");
        return;
    }

    /*
     *  First we get the file content ("name_of_glade_file.ui").
     *  And call the Builder call.
     *  Next, we define `window` var.
     */
    let ui = include_str!("../resources/glade/app.ui");
    let builder = Builder::new_from_string(ui);
    let main_window: Window = builder.get_object("main_window").expect(
        "Couldn't get main_window",
    );

    main_window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });


    let open_button = dialog::click_open;
    let about_button = dialog::click_about;
    let settings_button = settings::create_setting_window;

    /*
     *   Here we define `about` variable which implement
     *   "connect_clicked" method with `about_button` variable.
     */
    let about: Button = builder.get_object("about_btn").expect(
        "Couldn't get about_button",
    );

    let setting: Button = builder.get_object("setup_btn").expect(
        "Couldn't get setup_button",
    );


    open_button(&builder, &main_window);
    setting.connect_clicked(move |_| settings_button());
    // about_button and about = супер костыль.
    about.connect_clicked(move |x| about_button(x, &builder));

    /*
     *   Recursively shows a widget (any child widgets)
     *   and start the gtk main loop.
     */
    main_window.show_all();
    gtk::main();
}
