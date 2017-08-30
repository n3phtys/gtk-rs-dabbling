extern crate gtk;

use gtk::prelude::*;
#[macro_use] extern crate closet;



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

    button1.connect_clicked(clone_army!(
            [button1, button2, button3]
            move |_| {
            button1.set_visible(false);
            button2.set_visible(true);
            button3.set_visible(true);
            println!("Button 1 pressed");
        }));

    button2.connect_clicked(clone_army!(
            [button1, button2, button3]
            move |_| {
            button1.set_visible(true);
            button2.set_visible(false); // == GONE
            //should use grid for multiple buttons so they do not collapse
            button3.set_visible(true);
            println!("Button 2 pressed");
        }));

    button3.connect_clicked(clone_army!(
            [button1, button2, button3]
            move |_| {
            button1.set_visible(true);
            button2.set_visible(true);
            button3.set_visible(false);
            println!("Button 3 pressed");
        }));

    gtk::main();
}