extern crate gtk;

use gtk::prelude::*;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_title("Hello, World");
    window.show_all();

    gtk::main();
}
