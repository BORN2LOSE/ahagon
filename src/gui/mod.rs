use gtk;
use gio;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{ApplicationWindow, Builder, Button};

use std::env::args;

use core::dialog;
// use core::settings;

macro_rules! clone {
        (@param _) => ( _ );
        (@param $x:ident) => ( $x );
        ($($n:ident),+ => move || $body:expr) => (
            {
                $( let $n = $n.clone(); )+
                move || $body
            }
        );
        ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
            {
                $( let $n = $n.clone(); )+
                move |$(clone!(@param $p),)+| $body
            }
        );
}

pub fn build_gtk(application: &gtk::Application) {

    let glade_src = include_str!("../resources/glade/app.ui");
    let builder = Builder::new_from_string(glade_src);

    let main_window: ApplicationWindow = builder.get_object("main_window").expect(
        "Couldn't get main_window",
    );

    main_window.set_application(application);
    main_window.connect_delete_event(clone!(main_window => move |_, _| {
            main_window.destroy();
            Inhibit(false)
    }));

    let open_button = dialog::click_open;
    let about_button = dialog::click_about;
    // let settings_button = settings::create_setting_window;

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

    // open_button(&builder, &main_window);
    // setting.connect_clicked(move |_| settings_button());
    // about_button and about = супер костыль.
    about.connect_clicked(move |x| about_button(x, &builder));

    main_window.show_all();
}

pub fn launch_gtk() {
    let application =
        gtk::Application::new("com.github.builder_basics", gio::ApplicationFlags::empty())
            .expect("Initialization failed...");

    application.connect_startup(move |app| { build_gtk(app); });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
