use gtk::prelude::*;
use mlua::{UserData, UserDataFields};
pub struct InputOptions<'a> {
    pub widget: &'a gtk::Text,
}

impl UserData for InputOptions<'_> {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("value", |_, this| Ok(String::from(this.widget.text())));
        fields.add_field_method_set("value", |_, this, value: String| {
            Ok(this.widget.set_text(&value))
        });
    }
}
