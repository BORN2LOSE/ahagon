// Some work in progress.
use gtk::prelude::*;
use gtk::{Builder, Button, AboutDialog, FileChooserDialog, FileChooserAction, ResponseType, Window};

// Make moving clones into closures more convenient
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


/*
 *   Here we define `click_open` function which opening
 *   File Chooser Dialog.
 */
pub fn click_open(builder: &Builder, window: &Window) {
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
}


/*
 *   Here we define `click_about` function which impelements the logic
 *   that is responsible for pressing "About" button.
 */
pub fn click_about(button: &Button, builder: &Builder) {
    let dialog: AboutDialog = builder.get_object("about_dialog").expect(
        "Couldn't get dialog",
    );
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
