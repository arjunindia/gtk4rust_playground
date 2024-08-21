use mlua::{UserData, UserDataFields};
pub struct FetchOptions {
    pub response: String,
}

impl UserData for FetchOptions {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("body", |_, this| {
            let body = &this.response;
            Ok(body)
        });
    }
}
