i = 0
imageRef = nil
headingRef = nil

local render = function()
    return horizontal(
        button({ onclick = function()
    i = i - 1
    imageRef.url = "https://picsum.photos/600/400?random=" .. i
end }, "<"),
        image({ ref = function(ref) imageRef end }, "https://picsum.photos/600/400"),
        button({ onclick = function()
    i = i + 1
    imageRef.url = "https://picsum.photos/600/400?random=" .. i
end }, ">")
    )
end

_tree = render()
return _tree;
