use std::rc::Rc;

use mlua::{prelude::*, Lua};

pub fn patch(lua: Rc<Lua>) -> Result<(), LuaError> {
    lua.load(
        r#"
        heading = function(title)
            return {type="heading", title=title}
        end
        text = function(content)
            return {type="text", content=content}
        end
        image = function(url)
            return {type="image", url=url}
        end
        horizontal = function(...)
            return {type="horizontal", children={...}}
        end
        vertical = function(...)
            return {type="vertical", children={...}}
        end
-- Function to recursively print nested tables
function print_table(tbl, indent, done)
    -- Set default values for indent and done if not provided
    indent = indent or 0
    done = done or {}

    -- Helper function to create indentation
    local function indent_str(level)
        return string.rep("  ", level)
    end

    -- Function to print a single key-value pair
    local function print_pair(key, value)
        if type(value) == "table" then
            if done[value] then
                print(indent_str(indent) .. "[" .. tostring(key) .. "] = (already seen)")
            else
                done[value] = true
                print(indent_str(indent) .. "[" .. tostring(key) .. "] = {")
                print_table(value, indent + 1, done)
                print(indent_str(indent) .. "}")
            end
        else
            print(indent_str(indent) .. "[" .. tostring(key) .. "] = " .. tostring(value))
        end
    end

    -- Iterate over table elements and print each one
    for key, value in pairs(tbl) do
        print_pair(key, value)
    end
end
        "#,
    )
    .exec()
}
