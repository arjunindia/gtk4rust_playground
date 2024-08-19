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
                    label.set_selectable(true);
                    Ok(label.upcast())
                }
                "text" => {
                    let content = t.get::<_, String>("content")?;
                    let label = gtk::Label::new(Some(&content));
                    label.set_selectable(true);
                    Ok(label.upcast())
                }
                "button" => {
                    let button = gtk::Button::with_label("Click Me!");
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
                    let image = gtk::Image::from_pixbuf(Some(&pixbuf));
                    image.set_width_request(400);
                    image.set_height_request(300);
                    Ok(image.upcast())
                }
                "vertical" | "horizontal" => {
                    let container = if element == "horizontal" {
                        gtk::Box::new(gtk::Orientation::Horizontal, 0)
                    } else {
                        gtk::Box::new(gtk::Orientation::Vertical, 0)
                    };
                    container.set_homogeneous(false);
                    container.set_vexpand(false);
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
