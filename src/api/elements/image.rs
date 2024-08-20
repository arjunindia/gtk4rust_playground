use mlua::{UserData, UserDataFields};
pub struct ImageOptions<'a> {
    pub widget: &'a gtk::Picture,
    pub url: String,
}

impl UserData for ImageOptions<'_> {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("url", |_, this| Ok(this.url.clone()));
        fields.add_field_method_set("url", |_, this, url: String| {
            let result = reqwest::blocking::get(url.clone()).unwrap();
            let bytes = result.bytes().unwrap().to_vec();
            let bytes = gtk::glib::Bytes::from(&bytes.to_vec());
            let stream = gtk::gio::MemoryInputStream::from_bytes(&bytes);
            let pixbuf =
                gtk::gdk_pixbuf::Pixbuf::from_stream(&stream, gtk::gio::Cancellable::NONE).unwrap();
            this.widget.set_pixbuf(Some(&pixbuf));
            this.url = url;
            Ok(())
        });
    }
}
