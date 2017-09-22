extern crate gtk;

use gtk::prelude::*;

#[allow(unused_imports)]
use gtk::{AboutDialog, CheckMenuItem, IconSize, Image, Label, Menu, MenuBar, MenuItem, Window,
          WindowPosition, WindowType};

pub fn main_window() {

    // Initializing GTK Window.
    let window = Window::new(WindowType::Toplevel);

    window.set_title("Ahagon");
    window.set_position(WindowPosition::CenterAlways);
    window.set_default_geometry(750, 550);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);

    let menu = Menu::new();
    let setup_menu = Menu::new();
    let menu_bar = MenuBar::new();

    /*
     *   Items placed in Menu:
     */
    let file = MenuItem::new_with_label("File");
    let settings = MenuItem::new_with_label("Settings");
    let about = MenuItem::new_with_label("About");

    /*
     *  Item placed in "File" submenu with icons:
     */
    let file_item = MenuItem::new();
    let file_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let file_image = Image::new_from_file("resources/file.png");
    let file_label = Label::new(Some("Open File ..."));

    let folder_item = MenuItem::new();
    let folder_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let folder_image = Image::new_from_icon_name("folder-music-symbolic", IconSize::Menu.into());
    let folder_label = Label::new(Some("Open Directory ..."));
    // Exit action.
    let quit = MenuItem::new_with_label("Quit");

    /*
     *   Items placed in "Setting" submenu:
     */
    let theme = MenuItem::new();
    let theme_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let theme_image = Image::new_from_file("");
    let theme_label = Label::new(Some("Theme"));

    let proxy = MenuItem::new();
    let proxy_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let proxy_image = Image::new_from_file("");
    let proxy_label = Label::new(Some("Proxy"));

    file_box.pack_start(&file_image, false, false, 0);
    file_box.pack_start(&file_label, true, true, 0);
    file_item.add(&file_box);

    folder_box.pack_start(&folder_image, false, false, 0);
    folder_box.pack_start(&folder_label, true, true, 0);
    folder_item.add(&folder_box);

    theme_box.pack_start(&theme_image, false, false, 0);
    theme_box.pack_start(&theme_label, true, true, 0);
    theme.add(&theme_box);

    proxy_box.pack_start(&proxy_image, false, false, 0);
    proxy_box.pack_start(&proxy_label, true, true, 0);
    proxy.add(&proxy_box);

    /*
     * Here we add new items in `menu_bar`.
     */
    menu_bar.append(&file);
    // "File" include this items:
    menu.append(&file_item);
    menu.append(&folder_item);
    menu.append(&quit);

    menu_bar.append(&settings);
    // "Settings" include this items:
    setup_menu.append(&proxy);
    setup_menu.append(&theme);

    menu_bar.append(&about);



    /*
     * Add submenu.
     */
    file.set_submenu(Some(&menu));
    settings.set_submenu(Some(&setup_menu));

    quit.connect_activate(|_| { gtk::main_quit(); });

    let label = Label::new(Some("Drop you torrent file here or Open File."));

    v_box.pack_start(&menu_bar, false, false, 0);
    v_box.pack_start(&label, true, true, 0);

    about.connect_activate(move |_| {
        let p = AboutDialog::new();
        p.set_authors(&["Norman Ritchie"]);
        p.set_website(Some("https://github.com/BORN2LOSE"));
        // p.set_website_label(Some("gtk-rs"));
        p.set_title("About!");
        // p.set_transient_for(Some(&window));
        p.run();
        p.destroy();
    });

    window.add(&v_box);
    window.show_all();
}
