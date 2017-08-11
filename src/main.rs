extern crate gtk;

use gtk::prelude::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let button1 = gtk::Button::new_with_label("Click me 1!");
    let button2 = gtk::Button::new_with_label("Click me 2!");
    let button3 = gtk::Button::new_with_label("Click me 3!");

    {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        vbox.pack_start(&button1, true, true, 0);
        vbox.pack_start(&button2, false, false, 0);
        vbox.pack_start(&button3, true, true, 20);
        window.add(&vbox);
    }





    window.show_all();

    let button2cc = button2.clone();
        let button1c = button1.clone();
        button1.connect_clicked(move |_| {
            button1c.set_visible(false);
            button2cc.set_visible(true);
            println!("Button 1 pressed");
        });

    let mut button2c = button2.clone();
    //button2c.resize(300, 150);
    let button1cc = button1.clone();
        button2.connect_clicked(move |_| {
            button2c.set_visible(false); // == GONE
            //should use grid for multiple buttons so they do not collapse
            button1cc.set_visible(true);
            println!("Button 2 pressed");
        });

        let button3c = button3.clone();
        button3.connect_clicked(move |_| {
            button3c.set_visible(false);
            println!("Button 3 pressed");
        });

    gtk::main();
}