local counter = 0
local counterLabel = nil
local imageUrl = "https://picsum.photos/200/300"
local imagePicture = nil

local function updateCounter()
    counter = counter + 1
    counterLabel.label = "Counter: " .. counter
end

local function toggleImage()
    if imageUrl == "https://picsum.photos/200/300" then
        imageUrl = "https://picsum.photos/300/300"
    else
        imageUrl = "https://picsum.photos/200/300"
    end
    imagePicture.url = imageUrl
end

render = function()
    return vertical(
        heading("Broust Demo Page"),
        text("Welcome to this interactive demonstration of Broust's capabilities."),

        horizontal(
            { spacing = 10, marginTop = 20, marginBottom = 20 },
            button({ onclick = updateCounter }, "Increment Counter"),
            text({ ref = function(ref) counterLabel = ref end }, "Counter: " .. counter)
        ),

        input({
            placeholder = "Enter your name",
            width = 25,
            marginBottom = 10
        }),

        button({
            onclick = function()
                print("Hello button clicked!")
            end,
            marginBottom = 20
        }, "Say Hello"),

        image({
            ref = function(ref) imagePicture = ref end,
            url = imageUrl,
            width = 25,
            height = 50,
            marginBottom = 10
        }),

        button({ onclick = toggleImage }, "Toggle Image"),

        horizontal(
            { spacing = 20, marginTop = 20 },
            link({ url = "https://example.com" }, "Visit Example.com"),
            link({ url = "https://github.com" }, "Check out GitHub")
        ),

        vertical(
            { marginTop = 30 },
            heading("About Broust"),
            text("Broust is an experimental web browser that uses Lua for web content creation."),
            text("It aims to simplify web development by unifying markup, styling, and scripting under one language.")
        )
    )
end

return render()
