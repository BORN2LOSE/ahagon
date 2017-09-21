extern crate gtk;

pub mod start {
    use gtk;
    use gtk::prelude::*;
    use gtk::{Button, Window, WindowType};

    pub fn launch() {
        if gtk::init().is_err() {
            println!("Failed to initialize Ahagon GTK.");
            return;
        }

        let window = Window::new(WindowType::Toplevel);
        window.set_title("Ahagon");
        window.set_default_size(350, 170);
        let button = Button::new_with_label("Click on me!");
        window.add(&button);
        window.show_all();

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        button.connect_clicked(|_| {
            println!("Клик!");
        });

        gtk::main();
    }
}


fn main() {
    start::launch();
}
