pub mod gui {
    use gtk;
    use gtk::prelude::*;

    // Some work in progress.
    #[allow(unused_imports)]
    use gtk::{Builder, Button, MessageDialog, AboutDialog, FileChooserDialog, FileChooserAction,
              ResponseType, Window};

    // make moving clones into closures more convenient
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

    // Добавить окно для кнопки `About`.
    fn click_about(button: &Button, builder: &Builder) {
        let dialog: AboutDialog = builder.get_object("dialog").expect("Couldn't get dialog");
        if let Some(window) = button.get_toplevel().and_then(
            |w| w.downcast::<Window>().ok(),
        )
        {
            dialog.set_transient_for(Some(&window));
        }

        dialog.show();
        dialog.run();
        dialog.hide();
    }

    pub fn launch() {
        if gtk::init().is_err() {
            println!("Ahagon: failed to initialize GTK.");
            return;
        }

        let ui = include_str!("app.ui");
        let builder = Builder::new_from_string(ui);

        let window: Window = builder.get_object("main_window").expect(
            "Couldn't get main_window",
        );

        /*
         *   Open torrent file button
         */
        let open_file: Button = builder.get_object("open_btn").expect(
            "Couldn't get open_button",
        );
        open_file.connect_clicked(clone!(window => move |_| {
            let dialog = FileChooserDialog::new(Some("Choose a torrent file"),Some(&window),
                                                        FileChooserAction::Open);
            dialog.add_buttons(&[
                ("Open", ResponseType::Ok.into()),
                ("Cancel", ResponseType::Cancel.into())
            ]);

            dialog.set_select_multiple(true);
            dialog.run();
            let files = dialog.get_filenames();
            dialog.destroy();

            println!("Files: {:?}", files);
        }));

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
