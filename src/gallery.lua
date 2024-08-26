i = 0
imageRef = nil
headingRef = nil

local render = function()
    return horizontal(
        button({
            width = 20,
            height = 90,
            onclick = function()
                i = i - 1
                imageRef.url = "https://picsum.photos/600/400?random=" .. i
            end
        }, "<"),
        image({ width = 50, height = 90 }, { ref = function(ref) imageRef = ref end }, "https://picsum.photos/600/400"),
        button({
            width = 20,
            height = 90,
            onclick = function()
                i = i + 1
                imageRef.url = "https://picsum.photos/600/400?random=" .. i
            end
        }, ">")
    )
end

_tree = render()
return _tree;
