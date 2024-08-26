local i = 0
local imageRef = nil
local headingRef = nil
local render = function()
    return horizontal(
        vertical(
            heading({ ref = function(ref) headingRef = ref end }, "My Awesome Blog"),
            text("Welcome to my blog where I share exciting content and insights on various topics."),
            horizontal(
                link(
                    {
                        valign = "baselinecenter",
                        url =
                        "https://raw.githubusercontent.com/arjunindia/gtk4rust_playground/main/src/main.lua"
                    }, "Home"),
                link(
                    {
                        valign = "baselinecenter",
                        url =
                        "https://raw.githubusercontent.com/arjunindia/gtk4rust_playground/main/src/gallery.lua"
                    }, "Gallery"),
                link(
                    {
                        valign = "baselinecenter",
                        url =
                        "https://raw.githubusercontent.com/arjunindia/gtk4rust_playground/main/src/counter.lua"
                    }, "Counter"),
                text({ valign = "baselinecenter" }, "Contact"),
                button({
                    onclick = function()
                        print(i)
                        headingRef.label = i
                        imageRef.url = "https://picsum.photos/400/200?random=" .. i
                        i = i + 1
                    end
                }, "HIII")
            ),
            horizontal({
                    width = 100, spacing = 20 },
                vertical(
                    image({ ref = function(ref) imageRef = ref end }, "https://picsum.photos/400/200?random=" .. i),
                    vertical(
                        text("Amazing Blog Title 1"),
                        text({ max_width = 35 },
                            "A brief description of the first blog post. It covers interesting insights and provides valuable information.")
                    )
                ),
                vertical(
                    image("https://picsum.photos/400/200?random=2"),
                    vertical(
                        text("Intriguing Blog Title 2"),
                        text({ max_width = 65 },
                            "A summary of the second blog post. It dives into various topics and presents engaging content.")
                    )
                ),
                vertical(
                    image("https://picsum.photos/400/200?random=3"),
                    vertical(
                        text("Fascinating Blog Title 3"),
                        text({ max_width = 65 },
                            "An overview of the third blog post. It highlights important points and shares helpful tips.")
                    )
                )
            ),
            text(
                "Thank you for visiting my blog! Stay tuned for more updates and feel free to reach out if you have any questions.")
        )
    )
end

_tree = render()
return _tree;
