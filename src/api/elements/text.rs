use mlua::{UserData, UserDataFields};
pub struct TextOptions<'a> {
    pub widget: &'a gtk::Label,
}

impl UserData for TextOptions<'_> {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("label", |_, this| Ok(this.widget.label().to_string()));
        fields.add_field_method_set("label", |_, this, value: String| {
            Ok(this.widget.set_label(&value))
        });
    }
}
