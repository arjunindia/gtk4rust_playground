use super::api::elements;
use gtk::gdk::Display;
use gtk::prelude::*;
use mlua::prelude::*;
use std::error::Error;
use std::rc::Rc;
pub fn render(tree: &'static mut LuaValue<'static>) -> Result<gtk::Widget, Box<dyn Error>> {
    match tree {
        LuaValue::Table(t) => {
            let element = t.get::<_, String>("type")?;
            let properties = t.get::<_, LuaTable>("properties")?;
            let widget: Result<gtk::Widget, Box<dyn Error>> = match element.as_str() {
                "heading" => {
                    let title = t.get::<_, String>("title")?;
                    let label = Box::new(Rc::new(gtk::Label::new(Some(&title))));
                    label.set_selectable(true);
                    let interface = elements::heading::HeadingOptions {
                        widget: Box::leak(label.clone()),
                    };
                    let reffunc = properties.get::<_, LuaFunction>("ref").ok();
                    if let Some(func) = reffunc {
                        func.call::<elements::heading::HeadingOptions, ()>(interface)?;
                    } else {
                    }
                    Ok(<gtk::Label as Clone>::clone(&label.clone()).upcast())
                }
                "text" => {
                    let content = t.get::<_, String>("content")?;
                    let label = Box::new(Rc::new(gtk::Label::new(Some(&content))));
                    label.set_selectable(true);
                    let interface = elements::text::TextOptions {
                        widget: Box::leak(label.clone()),
                    };
                    let reffunc = properties.get::<_, LuaFunction>("ref").ok();
                    if let Some(func) = reffunc {
                        func.call::<elements::text::TextOptions, ()>(interface)?;
                    } else {
                    }

                    if let Some(width) = properties.get::<_, i32>("width").ok() {
                        label.set_width_chars(width);
                    }
                    if let Some(max_width) = properties.get::<_, i32>("max_width").ok() {
                        label.set_max_width_chars(max_width);
                    }

                    Ok(<gtk::Label as Clone>::clone(&label.clone()).upcast())
                }
                "link" => {
                    let content = t.get::<_, String>("content")?;
                    let url = t.get::<_, String>("url")?;
                    let label = Box::new(Rc::new(gtk::LinkButton::with_label(&url, &content)));
                    let interface = elements::link::LinkOptions {
                        widget: Box::leak(label.clone()),
                    };
                    let reffunc = properties.get::<_, LuaFunction>("ref").ok();
                    if let Some(func) = reffunc {
                        func.call::<elements::link::LinkOptions, ()>(interface)?;
                    } else {
                    }
                    let onclick = t.get::<_, LuaFunction>("onclick")?;
                    label.connect_activate_link(move |_| {
                        onclick.call::<_, ()>(()).unwrap();
                        gtk::glib::signal::Propagation::Stop
                    });

                    Ok(<gtk::LinkButton as Clone>::clone(&label.clone()).upcast())
                }
                "input" => {
                    let input = Box::new(Rc::new(gtk::Text::new()));

                    Ok(<gtk::Text as Clone>::clone(&input.clone()).upcast())
                }
                "button" => {
                    let content = t.get::<_, String>("content")?;
                    let button = Box::new(Rc::new(gtk::Button::with_label(&content)));

                    button.connect_clicked(move |_| {
                        let properties: LuaTable = t.get("properties").unwrap();
                        if let Some(func) = properties.get::<_, LuaFunction>("onclick").ok() {
                            func.call::<_, ()>(()).unwrap();
                        } else {
                            println!("'onclick' function is not set in Lua.");
                        }
                    });
                    let interface = elements::button::ButtonOptions {
                        widget: Box::leak(button.clone()),
                    };
                    let reffunc = properties.get::<_, LuaFunction>("ref").ok();
                    if let Some(func) = reffunc {
                        func.call::<elements::button::ButtonOptions, ()>(interface)?;
                    } else {
                    }

                    Ok(<gtk::Button as Clone>::clone(&button.clone()).upcast())
                }

                "image" => {
                    let url = t.get::<_, String>("url")?;
                    let result = reqwest::blocking::get(url.clone())?;
                    let bytes = result.bytes().unwrap().to_vec();
                    let bytes = gtk::glib::Bytes::from(&bytes.to_vec());
                    let stream = gtk::gio::MemoryInputStream::from_bytes(&bytes);
                    let pixbuf =
                        gtk::gdk_pixbuf::Pixbuf::from_stream(&stream, gtk::gio::Cancellable::NONE)?;
                    let image = Box::new(Rc::new(gtk::Picture::for_pixbuf(&pixbuf)));
                    let interface = elements::image::ImageOptions {
                        widget: Box::leak(image.clone()),
                        url,
                    };
                    let reffunc = properties.get::<_, LuaFunction>("ref").ok();
                    if let Some(func) = reffunc {
                        func.call::<elements::image::ImageOptions, ()>(interface)?;
                    } else {
                    }

                    Ok(<gtk::Picture as Clone>::clone(&image.clone()).upcast())
                }
                "vertical" | "horizontal" => {
                    let spacing = if let Some(spacing) = properties.get::<_, i32>("spacing").ok() {
                        spacing
                    } else {
                        0
                    };

                    let container = if element == "horizontal" {
                        gtk::Box::new(gtk::Orientation::Horizontal, spacing)
                    } else {
                        gtk::Box::new(gtk::Orientation::Vertical, spacing)
                    };

                    container.add_css_class("container");
                    if let Some(balanced) = properties.get::<_, bool>("balanced").ok() {
                        container.set_homogeneous(balanced);
                    } else {
                        container.set_homogeneous(false);
                    }

                    if let Some(baseline) = properties.get::<_, String>("baseline_position").ok() {
                        let baseline = match baseline.to_lowercase().as_str() {
                            "top" => gtk::BaselinePosition::Top,
                            "center" => gtk::BaselinePosition::Center,
                            "bottom" => gtk::BaselinePosition::Bottom,
                            _ => gtk::BaselinePosition::Top,
                        };
                        container.set_baseline_position(baseline);
                    } else {
                        container.set_baseline_position(gtk::BaselinePosition::Top);
                    }

                    let children = t.get::<_, Vec<LuaValue>>("children")?;
                    for child in children {
                        let rendered_child = render(Box::leak(Box::new(child)))?;
                        container.append(&rendered_child);
                    }
                    Ok(container.upcast())
                }
                _ => Err(format!("Invalid element : {}", element).into()),
            };
            match widget {
                Ok(widget) => {
                    // default css class
                    widget.add_css_class(&element);

                    // size

                    if let Some(height) = properties.get::<_, i32>("height").ok() {
                        widget.set_height_request(height);
                    }

                    if let Some(width) = properties.get::<_, i32>("width").ok() {
                        widget.set_width_request(width);
                    }
                    // margin
                    if let Some(margin_top) = properties.get::<_, i32>("marginTop").ok() {
                        widget.set_margin_top(margin_top);
                    }
                    if let Some(margin_bottom) = properties.get::<_, i32>("marginBottom").ok() {
                        widget.set_margin_top(margin_bottom);
                    }
                    if let Some(margin_left) = properties.get::<_, i32>("marginLeft").ok() {
                        widget.set_margin_start(margin_left);
                    }
                    if let Some(margin_right) = properties.get::<_, i32>("marginRight").ok() {
                        widget.set_margin_end(margin_right);
                    }
                    // alignment
                    if let Some(halign) = properties.get::<_, String>("halign").ok() {
                        let halign = match halign.to_lowercase().as_str() {
                            "start" => gtk::Align::Start,
                            "end" => gtk::Align::End,
                            "fill" => gtk::Align::Fill,
                            "center" => gtk::Align::Center,
                            "baseline" => gtk::Align::Baseline,
                            "baselinefill" => gtk::Align::BaselineFill,
                            "baselinecenter" => gtk::Align::BaselineCenter,
                            _ => gtk::Align::Start,
                        };
                        widget.set_halign(halign);
                    } else {
                        widget.set_halign(gtk::Align::Start);
                    }

                    if let Some(valign) = properties.get::<_, String>("valign").ok() {
                        let valign = match valign.to_lowercase().as_str() {
                            "start" => gtk::Align::Start,
                            "end" => gtk::Align::End,
                            "fill" => gtk::Align::Fill,
                            "center" => gtk::Align::Center,
                            "baseline" => gtk::Align::Baseline,
                            "baselinefill" => gtk::Align::BaselineFill,
                            "baselinecenter" => gtk::Align::BaselineCenter,
                            _ => gtk::Align::Start,
                        };
                        widget.set_valign(valign);
                    } else {
                        widget.set_valign(gtk::Align::Start);
                    }
                    // expand
                    if let Some(hexpand) = properties.get::<_, bool>("hexpand").ok() {
                        widget.set_hexpand(hexpand);
                    } else {
                        widget.set_hexpand(false)
                    }
                    if let Some(vexpand) = properties.get::<_, bool>("vexpand").ok() {
                        widget.set_vexpand(vexpand);
                    } else {
                        widget.set_vexpand(false)
                    }

                    Ok(widget)
                }
                Err(e) => Err(e),
            }
        }
        _ => Err("Render function returns invalid result!".into()),
    }
}
