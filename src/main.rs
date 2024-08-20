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
    api::patch(*lua.clone()).unwrap();
    let _ = lua.load(r#"
        i = 0
        imageRef = nil
        headingRef = nil
        render = function()
            return horizontal(
                vertical(
                    heading({ref = function(ref) headingRef = ref end},"My Awesome Blog"),
                    text("Welcome to my blog where I share exciting content and insights on various topics."),
                    horizontal(
                        text({valign="baselinecenter"},"Home"),
                        text({valign="baselinecenter"},"About"),
                        text({valign="baselinecenter"},"Categories"),
                        text({valign="baselinecenter"},"Contact"),
                        button({onclick=function()
                                print(i)
                                headingRef.label = i
                                imageRef.url = "https://picsum.photos/400/200?random=" .. i
                                i=i+1
                            end},"HIII")
                    ),
                    horizontal({
                        width=1920,spacing=20},
                        vertical(
                            {height = 100},
                            image({ref=function(ref) imageRef = ref  end},"https://picsum.photos/400/200?random=" .. i),
                            vertical(
                                text("Amazing Blog Title 1"),
                                text({max_width=35},"A brief description of the first blog post. It covers interesting insights and provides valuable information.")
                            )
                        ),
                        vertical(
                            {height = 100},
                            image("https://picsum.photos/400/200?random=2"),
                            vertical(
                                text("Intriguing Blog Title 2"),
                                text({max_width=65},"A summary of the second blog post. It dives into various topics and presents engaging content.")
                            )
                        ),
                        vertical(
                            {height=100},
                            image("https://picsum.photos/400/200?random=3"),
                            vertical(
                                text("Fascinating Blog Title 3"),
                                text({max_width=65},"An overview of the third blog post. It highlights important points and shares helpful tips.")
                            )
                        )
                    ),
                    text("Thank you for visiting my blog! Stay tuned for more updates and feel free to reach out if you have any questions.")
                )
            )
        end
        "#
    ).exec().unwrap();
    let render = Box::leak(lua.clone())
        .globals()
        .get::<_, LuaFunction<'static>>("render")
        .unwrap();
    let tree = render.call::<_, LuaValue>(()).unwrap();
    lua.load("print_table(render())").exec().unwrap();
    let dom = view::render(Box::leak(Box::new(tree))).unwrap();
    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .min_content_width(1920)
        .min_content_height(900)
        .margin_top(20)
        .margin_start(10)
        .margin_end(10)
        .margin_bottom(10)
        .child(&dom)
        .build();

    // Create a window and set the title
    (&*window.borrow()).set_child(Some(&scrolled_window));
    // Present window
    (&*window.borrow()).present();
}
