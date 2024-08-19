use gtk::prelude::*;
use mlua::prelude::*;
use std::error::Error;
pub fn render(tree: &'static mut LuaValue<'static>) -> Result<gtk::Widget, Box<dyn Error>> {
    match tree {
        LuaValue::Table(t) => {
            let element = t.get::<_, String>("type")?;
            match element.as_str() {
                "heading" => {
                    let title = t.get::<_, String>("title")?;
                    let label = gtk::Label::new(Some(&title));
                    label.add_css_class("heading");
                    label.set_selectable(true);
                    label.set_halign(gtk::Align::Start);
                    label.set_valign(gtk::Align::Start);
                    label.set_vexpand(false);
                    label.set_hexpand(false);

                    Ok(label.upcast())
                }
                "text" => {
                    let content = t.get::<_, String>("content")?;
                    let label = gtk::Label::new(Some(&content));
                    label.add_css_class("text");
                    label.set_selectable(true);
                    label.set_halign(gtk::Align::Start);
                    label.set_valign(gtk::Align::Start);
                    label.set_vexpand(false);
                    label.set_hexpand(false);

                    Ok(label.upcast())
                }
                "button" => {
                    let button = gtk::Button::with_label("Click Me!");
                    button.add_css_class("button");
                    button.set_halign(gtk::Align::Start);
                    button.set_valign(gtk::Align::Start);
                    button.set_vexpand(false);
                    button.set_hexpand(false);

                    button.connect_clicked(move |_| {
                        let properties: LuaTable = t.get("properties").unwrap();
                        let onclick: Option<LuaFunction> = properties.get("onclick").ok();

                        if let Some(func) = onclick {
                            func.call::<_, ()>(()).unwrap();
                        } else {
                            println!("'onclick' function is not set in Lua.");
                        }
                    });
                    Ok(button.upcast())
                }

                "image" => {
                    let url = t.get::<_, String>("url")?;
                    let result = reqwest::blocking::get(url)?;
                    let bytes = result.bytes().unwrap().to_vec();
                    let bytes = gtk::glib::Bytes::from(&bytes.to_vec());
                    let stream = gtk::gio::MemoryInputStream::from_bytes(&bytes);
                    let pixbuf =
                        gtk::gdk_pixbuf::Pixbuf::from_stream(&stream, gtk::gio::Cancellable::NONE)?;
                    let image = gtk::Picture::for_pixbuf(&pixbuf);

                    image.add_css_class("image");
                    image.set_halign(gtk::Align::Start);
                    image.set_valign(gtk::Align::Start);
                    image.set_vexpand(false);
                    image.set_hexpand(false);
                    Ok(image.upcast())
                }
                "vertical" | "horizontal" => {
                    let container = if element == "horizontal" {
                        gtk::Box::new(gtk::Orientation::Horizontal, 0)
                    } else {
                        gtk::Box::new(gtk::Orientation::Vertical, 0)
                    };

                    container.add_css_class("container");
                    container.add_css_class(&element);
                    container.set_halign(gtk::Align::Start);
                    container.set_valign(gtk::Align::Start);
                    container.set_homogeneous(false);
                    container.set_vexpand(false);
                    container.set_hexpand(false);
                    container.set_baseline_position(gtk::BaselinePosition::Top);
                    let children = t.get::<_, Vec<LuaValue>>("children")?;
                    for child in children {
                        let rendered_child = render(Box::leak(Box::new(child)))?;
                        container.append(&rendered_child);
                    }
                    Ok(container.upcast())
                }
                _ => Err(format!("Invalid element : {}", element).into()),
            }
        }
        _ => Err("Render function returns invalid result!".into()),
    }
}
