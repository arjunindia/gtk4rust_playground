use gtk::{glib, Application, ApplicationWindow};
use gtk::{prelude::*, ScrolledWindow};
use mlua::prelude::*;
use std::{cell::RefCell, rc::Rc};
mod api;

const APP_ID: &str = "org.broust.browser";

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
    let window = Rc::new(RefCell::new(
        ApplicationWindow::builder()
            .application(app)
            .title("Broust")
            .default_width(800)
            .default_height(600)
            .build(),
    ));
    let root = api::tree::create_container_node("vertical");
    api::render(Rc::clone(&lua), root.clone());
    let _ = lua.load(r#"
        render(function()
    -- Header Section
    heading("My Awesome Blog")
    text("Welcome to my blog where I share exciting content and insights on various topics.")

    -- Navigation Bar
    horizontal(function()
        vertical(function()
            text("Home")
            text("About")
            text("Categories")
            text("Contact")
        end)
    end)

    -- Blog Posts Section
    horizontal(function()
        vertical(function()
            image("https://picsum.photos/400/200?random=1")
            text("Amazing Blog Title 1")
            text("A brief description of the first blog post. It covers interesting insights and provides valuable information.")
        end)
        vertical(function()
            image("https://picsum.photos/400/200?random=2")
            text("Intriguing Blog Title 2")
            text("A summary of the second blog post. It dives into various topics and presents engaging content.")
        end)
        vertical(function()
            image("https://picsum.photos/400/200?random=3")
            text("Fascinating Blog Title 3")
            text("An overview of the third blog post. It highlights important points and shares helpful tips.")
        end)
    end)

    -- Footer Section
    text("Thank you for visiting my blog! Stay tuned for more updates and feel free to reach out if you have any questions.")
end)"#
    ).exec().unwrap();
    let dom = api::tree::create_gtk_widget_from_node(root);
    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .min_content_width(360)
        .min_content_height(100)
        .child(&dom)
        .build();

    // Create a window and set the title
    (&*window.borrow()).set_child(Some(&scrolled_window));
    // Present window
    (&*window.borrow()).present();
}
