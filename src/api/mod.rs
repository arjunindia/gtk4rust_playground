use std::{cell::RefCell, rc::Rc};

use gtk::{prelude::*, ApplicationWindow, Box, Button, Label, ScrolledWindow};
use mlua::{prelude::*, Function};

use self::tree::Node;
pub mod tree;
mod widgets;
pub fn render(lua: Rc<Lua>, root: Rc<RefCell<Node>>) {
    let list_box = Rc::new(RefCell::new(Box::new(gtk::Orientation::Vertical, 0)));

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

    // Render function
    let root_clone = Rc::clone(&root);
    let render = lua
        .create_function(move |lua, func: Function| {
            let root_copy = Rc::clone(&root_clone);

            //TODO: (broken as well) instead of passing list box, pass a tree node. let the widgets
            //build a tree and at the end traverse the tree to make the UI.
            widgets::init_widgets(lua, root_copy).unwrap();
            func.call::<_, ()>(())?;
            println!("{:?}", root_clone);
            Ok(())
        })
        .unwrap();
    lua.globals().set("render", render).unwrap();
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
            .load("greet('Arjun') render(function() heading('Hi') end)")
            .exec()
            .unwrap();

        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    list_box.borrow_mut().append(&button);
}
