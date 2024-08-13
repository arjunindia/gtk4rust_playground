use gtk::{glib, Application, ApplicationWindow, Button, Label, ListBox, ScrolledWindow};
use gtk::{prelude::*, PolicyType};
use mlua::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let lua = Rc::new(Lua::new());
    let map_table = lua.create_table().unwrap();
    map_table.set(1, "one").unwrap();
    map_table.set("two", 2).unwrap();
    let list_box = Rc::new(RefCell::new(ListBox::new()));

    lua.globals().set("map_table", map_table).unwrap();

    let list_box_clone = Rc::clone(&list_box);
    let greet = lua
        .create_function(move |_, name: String| {
            let list_box = list_box_clone.borrow_mut();
            let label = Label::new(Some(&name));
            list_box.append(&label);

            println!("Hello, {}!", name);
            Ok(())
        })
        .unwrap();
    lua.globals().set("greet", greet).unwrap();

    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Clone Lua for use in the closure
    let lua_clone = Rc::clone(&lua);
    button.connect_clicked(move |button| {
        lua_clone
            .load("for k,v in pairs(map_table) do print(k,v) greet('Arjun') end")
            .exec()
            .unwrap();

        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    for number in 0..=100 {
        let label = Label::new(Some(&number.to_string()));
        list_box.borrow_mut().append(&label);
    }

    list_box.borrow_mut().append(&button);

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .min_content_width(360)
        .min_content_height(100)
        .child(&*list_box.borrow())
        .build();

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&scrolled_window)
        .build();

    // Present window
    window.present();
}
