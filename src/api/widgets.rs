use mlua::{Error, Function, Lua};
use std::{cell::RefCell, rc::Rc};

use super::tree;
// todo: Fix this, currently vertical/horizonal functions dont add container nodes to the tree and
// actually do nothing except call the arg func.
pub fn init_widgets(lua: &Lua, parent: Rc<RefCell<tree::Node>>) -> Result<(), Error> {
    let parent_copy = Rc::clone(&parent);
    let heading = lua.create_function(move |_, content: String| {
        parent_copy
            .borrow_mut()
            .append_child(tree::create_leaf_node(&content, "heading"));
        println!("{}", content);
        Ok(())
    })?;
    lua.globals().set("heading", heading).unwrap();

    let parent_copy = Rc::clone(&parent);
    let text = lua.create_function(move |_, content: String| {
        parent_copy
            .borrow_mut()
            .append_child(tree::create_leaf_node(&content, "text"));

        println!("{}", content);
        Ok(())
    })?;
    lua.globals().set("text", text)?;

    let parent_copy = Rc::clone(&parent);
    let image = lua.create_function(move |_, content: String| {
        parent_copy
            .borrow_mut()
            .append_child(tree::create_leaf_node(&content, "image"));

        println!("image: {}", content);
        Ok(())
    })?;
    lua.globals().set("image", image)?;
    let vertical = lua.create_function(|_, func: Function| {
        func.call::<_, ()>(()).unwrap();
        Ok(())
    })?;

    lua.globals().set("vertical", vertical)?;
    let horizontal = lua.create_function(|_, func: Function| {
        func.call::<_, ()>(()).unwrap();
        Ok(())
    })?;

    lua.globals().set("horizontal", horizontal)?;
    Ok(())
}
