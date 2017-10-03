use gtk;
use gtk::prelude::*;
use gtk::{Builder, Button, Window};

pub struct SettingsDialog {
    setup_window: gtk::Window,
    proxy_btn: gtk::Button,
    theme_btn: gtk::Button,
}

impl SettingsDialog {
    pub fn new(builder: &Builder) -> SettingsDialog {
        // code here next time.
    }
}
