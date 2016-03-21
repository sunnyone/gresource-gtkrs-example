extern crate gtk;

use gtk::prelude::*;

mod static_resource;

fn main() {
    gtk::init().unwrap();
     
    static_resource::init(); 
 
    let builder = gtk::Builder::new_from_resource("/org/example/ExampleApp/main.ui");
     
    let window : gtk::Window = builder.get_object("window1").unwrap();
    
    let entry1 : gtk::Entry = builder.get_object("entry1").unwrap();
    let button1 : gtk::Button = builder.get_object("button1").unwrap();
    button1.connect_clicked(move |_| {
        let text = entry1.get_text().unwrap();
        println!("Text: {}", text);
    });
    
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    
    window.show_all();
    gtk::main();
}
