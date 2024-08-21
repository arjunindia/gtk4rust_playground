use mlua::{UserData, UserDataFields};
pub struct LinkOptions<'a> {
    pub widget: &'a gtk::LinkButton,
}

impl UserData for LinkOptions<'_> {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("url", |_, this| Ok(this.widget.uri().to_string()));
        fields.add_field_method_set("url", |_, this, value: String| {
            Ok(this.widget.set_uri(&value))
        });
    }
}
