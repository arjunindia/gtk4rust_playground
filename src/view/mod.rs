use gtk::prelude::*;
use std::error::Error;

use mlua::prelude::*;
pub fn render(tree: LuaValue) -> Result<gtk::Widget, Box<dyn Error>> {
    match tree {
        LuaValue::Table(t) => {
            let element = t.get::<_, String>("type").unwrap();
            match element.as_str() {
                "heading" => {
                    let title = t.get::<_, String>("title")?;
                    Ok(gtk::Label::new(Some(&title)).upcast())
                }
                "text" => {
                    let content = t.get::<_, String>("content")?;
                    Ok(gtk::Label::new(Some(&content)).upcast())
                }
                "image" => {
                    let url = t.get::<_, String>("url")?;
                    let result = reqwest::blocking::get(url).unwrap();
                    let bytes = result.bytes().unwrap().to_vec();
                    let bytes = gtk::glib::Bytes::from(&bytes.to_vec());
                    let stream = gtk::gio::MemoryInputStream::from_bytes(&bytes);
                    let pixbuf =
                        gtk::gdk_pixbuf::Pixbuf::from_stream(&stream, gtk::gio::Cancellable::NONE)
                            .unwrap();
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
                    let children = t.get::<_, Vec<LuaValue>>("children")?;
                    for child in children {
                        let rendered_child = render(child)?;
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

/*
 *
 *
use rlua::{Lua, Value};

fn parse_lua_table(lua: &Lua, table: Value) -> Result<gtk::Widget, Box<dyn std::error::Error>> {
    match table {
        Value::Table(table) => {
            let mut children = Vec::new();
            for pair in table.pairs::<String, Value>() {
                let (key, value) = pair?;
                match key.as_str() {
                    "type" => {
                        let widget_type = value.as_str()?;
                        let widget = match widget_type {
                            "heading" => {
                                let title = table.get::<_, String>("title")?;
                                gtk::Label::new(Some(&title)).upcast()
                            }
                            "text" => {
                                let content = table.get::<_, String>("content")?;
                                gtk::Label::new(Some(&content)).upcast()
                            }
                            "image" => {
                                let url = table.get::<_, String>("url")?;
                                let image = gtk::Image::new_from_file(&url);
                                image.upcast()
                            }
                            "horizontal" | "vertical" => {
                                let container = if widget_type == "horizontal" {
                                    gtk::Box::new(gtk::Orientation::Horizontal, 0)
                                } else {
                                    gtk::Box::new(gtk::Orientation::Vertical, 0)
                                };

                                if let Some(Value::Table(children_table)) = table.get::<_, Option<Value>>("children")? {
                                    for child in children_table.pairs::<String, Value>() {
                                        let (_, child_value) = child?;
                                        let child_widget = parse_lua_table(lua, child_value)?;
                                        container.append(&child_widget);
                                    }
                                }
                                container.upcast()
                            }
                            _ => return Err("Unknown widget type".into()),
                        };
                        children.push(widget);
                    }
                    _ => return Err("Unknown key".into()),
                }
            }
            // Assuming you want to return the first widget in the children list
            Ok(children.into_iter().next().unwrap())
        }
        _ => Err("Not a table".into()),
    }
}

 * */
