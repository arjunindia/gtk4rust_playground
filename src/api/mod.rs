pub mod elements;
use std::rc::Rc;

use mlua::{prelude::*, Lua};

pub fn patch(lua: Rc<Lua>) -> Result<(), LuaError> {
    let fetch = lua.create_function(|_, url: String| {
        let result = reqwest::blocking::get(url.clone()).unwrap();
        let r = result.text().unwrap();
        let resp = elements::fetch::FetchOptions {
            response: r.clone(),
        };
        println!("{url},{}", r);
        Ok(resp)
    })?;
    lua.globals().set("fetch", fetch)?;
    lua.load(
        r#"
        heading = function(arg1, title)
            local properties, actual_title
            if type(arg1) == "table" then
                properties = arg1
                actual_title = title
            else
                properties = {}
                actual_title = arg1
            end
            return {type = "heading", title = actual_title, properties = properties}
        end

        text = function(arg1, content)
            local properties, actual_content
            if type(arg1) == "table" then
                properties = arg1
                actual_content = content
            else
                properties = {}
                actual_content = arg1
            end
            return {type = "text", content = actual_content, properties = properties}
        end

        input = function(arg1, content)
            local properties, actual_content
            if type(arg1) == "table" then
                properties = arg1
                actual_content = content
            else
                properties = {}
                actual_content = arg1
            end
            return {type = "input", content = actual_content, properties = properties}
        end

       link = function(arg1, content)
            local properties, actual_content
            if type(arg1) == "table" then
                properties = arg1
                actual_content = content
            else
                properties = {}
                actual_content = arg1
            end
            return {type = "link", content = actual_content,onclick=function() print("link click") local r,e = load(fetch(properties.url).body) ok,ren = pcall(r) window(ren) end ,url = properties.url, properties = properties}
        end



        button = function(arg1, content)
            local properties, actual_content
            if type(arg1) == "table" then
                properties = arg1
                actual_content = content
            else
                properties = {}
                actual_content = arg1
            end
            return {type = "button", content = actual_content, properties = properties}
        end

        image = function(arg1, url)
            local properties, actual_url
            if type(arg1) == "table" then
                properties = arg1
                actual_url = url
            else
                properties = {}
                actual_url = arg1
            end
            return {type = "image", url = actual_url, properties = properties}
        end

        horizontal = function(...)
            local args = {...}
            local properties = {}
            local children = {}

            if type(args[1]) == "table" and not args[1].type then
                -- If the first table does not have a "type" property, it's properties
                properties = args[1]
                table.remove(args, 1)  -- Remove properties from arguments
            end

            -- Collect remaining arguments as children
            for _, child in ipairs(args) do
                if type(child) == "table" and child.type then
                    table.insert(children, child)
                end
            end

            return {type = "horizontal", properties = properties, children = children}
        end

        vertical = function(...)
            local args = {...}
            local properties = {}
            local children = {}

            if type(args[1]) == "table" and not args[1].type then
                -- If the first table does not have a "type" property, it's properties
                properties = args[1]
                table.remove(args, 1)  -- Remove properties from arguments
            end

            -- Collect remaining arguments as children
            for _, child in ipairs(args) do
                if type(child) == "table" and child.type then
                    table.insert(children, child)
                end
            end

            return {type = "vertical", properties = properties, children = children}
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
