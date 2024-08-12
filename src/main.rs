extern crate copypasta;
use copypasta::{ClipboardContext, ClipboardProvider};
//use std::io;
use gtk::{Application, ApplicationWindow, Button};
use gtk::prelude::*;

const APP_ID: &str = "org.gtk_rs.ClipX";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();
    // let mut line = String::new();
    // println!("Input some number ---- {}", ctx.get_contents().unwrap());
    // io::stdin().read_line(&mut line).unwrap();
    // println!("{}", line.as_str());
    // match line.as_str() {
    //     "1" => println!("{}", ctx.get_contents().unwrap()),
    //     "2" => println!("2 {}", line),
    //     _ => println!("invalid choices")
    // }
    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);
    // Run the application
    app.run();
}

fn show_clipboard(_: &Button) {
    let mut ctx = ClipboardContext::new().unwrap();
    println!("{}", ctx.get_contents().unwrap());
}

fn build_ui(app: &Application) {
    let texture_container = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .halign(gtk::Align::Center)
        .spacing(24)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    let copy_btn = gtk::Button::builder().label("Show").build();
    copy_btn.connect_clicked(show_clipboard);
    texture_container.append(&copy_btn);
    window.set_child(Some(&texture_container));
    window.present();
}

