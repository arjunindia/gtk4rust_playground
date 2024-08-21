counter = 0
labelRef = nil

local render = function()
    return vertical(
        text({ ref = function(ref) labelRef = ref end }, "Counter: " .. counter),
        button({
            onclick = function()
                counter = counter + 1
                labelRef.label = "Counter: " .. counter
            end
        }, "Increment Counter")
    )
end

_tree = render()
return _tree
