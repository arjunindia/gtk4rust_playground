use gtk::gdk::Display;
use gtk::{glib, Application, ApplicationWindow};
use gtk::{prelude::*, ScrolledWindow};
use mlua::prelude::*;
use std::{cell::RefCell, rc::Rc};
mod api;
mod view;
const APP_ID: &str = "org.broust.browser";

fn main() -> glib::ExitCode {
    let args: Vec<_> = std::env::args().collect();

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| load_css());
    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run_with_args(&args)
}
fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = gtk::CssProvider::new();
    provider.load_from_data(include_str!("style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
fn build_ui(app: &Application) {
    let lua = Box::new(Rc::new(Lua::new()));
    let window = Rc::new(RefCell::new(
        ApplicationWindow::builder()
            .application(app)
            .title("Broust")
            .default_width(800)
            .default_height(600)
            .build(),
    ));
    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    container.set_valign(gtk::Align::Fill);
    let container = Rc::new(RefCell::new(container));
    let container_copy = container.clone();
    let b_lua = lua.clone();
    let render_child = b_lua
        .create_function(move |_, tree: LuaValue| {
            let container_copy = container_copy.borrow_mut();
            container_copy.remove(&container_copy.last_child().unwrap());
            let mut tree = Box::leak(Box::new(tree.clone()));
            println!("{:?}", tree);
            let child = view::render(&mut tree).unwrap();
            container_copy.prepend(&child);
            Ok(())
        })
        .unwrap();
    lua.globals().set("window", render_child);
    api::patch(*lua.clone()).unwrap();

    let _ = lua.load(r#"
        i = 0
        imageRef = nil
        headingRef = nil
        render = function()
            return horizontal({halign="fill",valign="fill"},
                    input({halign="fill",width=80}),
                    link({url = "https://raw.githubusercontent.com/arjunindia/gtk4rust_playground/main/src/main.lua"},"Go!")
                   )
                end
        "#
    ).exec().unwrap();
    let binding = Box::leak(lua.clone());
    let render = binding.globals().get::<_, LuaFunction>("render").unwrap();
    let tree = render.call::<_, LuaValue>(()).unwrap();
    lua.load("print_table(render())").exec().unwrap();
    let dom = view::render(&mut Box::new(tree)).unwrap();
    container.borrow().append(&dom);

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .max_content_width(800)
        .min_content_width(800)
        .max_content_height(600)
        .min_content_height(600)
        .margin_top(20)
        .margin_start(10)
        .margin_end(10)
        .margin_bottom(10)
        .child(&*container.borrow())
        .build();
    // Create a window and set the title
    (&*window.borrow()).set_child(Some(&scrolled_window));
    // Present window
    (&*window.borrow()).present();
}
