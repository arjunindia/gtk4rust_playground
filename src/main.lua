local render = function()
    return vertical(
        { vexpand = true },
        heading("Arjun's Awesome Blog"),
        text("Welcome to my blog where I share exciting content and insights on various topics."),
        horizontal(
            { spacing = 10 },
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
        ),
        vertical({
                width = 100, spacing = 5 },
            vertical(
                image(
                    { width = 50, height = 30 },
                    "https://picsum.photos/400/200?random"
                ),
                vertical(
                    text("Amazing Blog Title 1"),
                    text({ max_width = 35 },
                        "A brief description of the first blog post. It covers interesting insights and provides valuable information.")
                )
            ),
            vertical(
                image(
                    { width = 50, height = 30 },
                    "https://picsum.photos/400/200?random"
                ),
                vertical(
                    text("Intriguing Blog Title 2"),
                    text({ max_width = 65 },
                        "A summary of the second blog post. It dives into various topics and presents engaging content.")
                )
            ),
            vertical(
                image(
                    { width = 50, height = 30 },
                    "https://picsum.photos/400/200?random"
                ),
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
end

local _tree = render()
return _tree;
