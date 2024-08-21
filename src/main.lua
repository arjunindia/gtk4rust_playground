i = 0
imageRef = nil
headingRef = nil
local render = function()
    return horizontal(
        vertical(
            heading({ ref = function(ref) headingRef = ref end }, "My Awesome Blog"),
            text("Welcome to my blog where I share exciting content and insights on various topics."),
            horizontal(
                text({ valign = "baselinecenter" }, "Home"),
                text({ valign = "baselinecenter" }, "About"),
                text({ valign = "baselinecenter" }, "Categories"),
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
                    width = 1920, spacing = 20 },
                vertical(
                    { height = 100 },
                    image({ ref = function(ref) imageRef = ref end }, "https://picsum.photos/400/200?random=" .. i),
                    vertical(
                        text("Amazing Blog Title 1"),
                        text({ max_width = 35 },
                            "A brief description of the first blog post. It covers interesting insights and provides valuable information.")
                    )
                ),
                vertical(
                    { height = 100 },
                    image("https://picsum.photos/400/200?random=2"),
                    vertical(
                        text("Intriguing Blog Title 2"),
                        text({ max_width = 65 },
                            "A summary of the second blog post. It dives into various topics and presents engaging content.")
                    )
                ),
                vertical(
                    { height = 100 },
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
