use gtk::prelude::*;
use mlua::{UserData, UserDataFields};
pub struct DropDownOptions<'a> {
    pub widget: &'a gtk::DropDown,
    pub vect: Vec<String>,
}

impl UserData for DropDownOptions<'_> {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("value", |_, this| {
            let idx: usize = this.widget.selected() as usize;
            let str = this.vect.get(idx).unwrap();
            Ok(str.to_owned())
        });
        fields.add_field_method_set("selected", |_, this, value: u32| {
            Ok(this.widget.set_selected(value))
        });
    }
}
