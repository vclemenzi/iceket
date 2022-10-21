mod interface;

extern crate gtk;
use gtk::{prelude::*, Inhibit, Window, WindowType};

fn main() {
    gtk::init().unwrap();

    let window = Window::new(WindowType::Toplevel);

    interface::build(&window);

    window.show_all();

    window.set_default_size(600, 700);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}