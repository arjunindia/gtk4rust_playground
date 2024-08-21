i = 0
imageRef = nil
headingRef = nil
left = function()
    i = i - 1
    imageRef.url = "https://picsum.photos/600/400?random=" .. i
end
right = function()
    i = i + 1
    imageRef.url = "https://picsum.photos/600/400?random=" .. i
end
local render = function()
    return horizontal(
        button({ onclick = left }, "<"),
        image({ ref = imageRef }, "https://picsum.photos/600/400"),
        button({ onclick = right }, ">")
    )
end

_tree = render()
return _tree;
