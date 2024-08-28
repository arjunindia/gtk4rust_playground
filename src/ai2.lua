-- Main render function that returns the root element of the page
function render()
    -- State management for the button click
    local buttonClicked = false

    -- Function to handle button click
    local function handleClick()
        buttonClicked = not buttonClicked
        -- Update the button text based on the click state
        buttonRef:setLabel(buttonClicked and "Clicked!" or "Click me!")
    end

    -- Reference for the button element
    local buttonRef

    -- Page layout
    return vertical(
    -- Header
        heading("Welcome to Broust Webpage"),

        -- Text block
        text("This is a simple webpage using Broust API with Lua."),

        -- Horizontal container for the button and image
        horizontal(
        -- Button with click event
            button(
                { width = 50, marginTop = 20, onclick = handleClick, ref = function(ref) buttonRef = ref end },
                "Click me!"
            ),

            -- Image with a placeholder URL
            image(
                { width = 50, marginTop = 20 },
                "https://via.placeholder.com/150"
            )
        ),

        -- Input field
        input(
            { width = 100, marginTop = 20, placeholder = "Type something here..." }
        ),

        -- Hyperlink
        link(
            { url = "https://www.example.com", marginTop = 20 },
            "Visit Example.com"
        )
    )
end

-- Fetch data from an external API
local function fetchData()
    local response = fetch("https://api.example.com/data")
    -- Process the response (e.g., update state or UI elements)
    -- (Implementation of response handling will depend on the API structure and needs)
end

-- Call fetchData() when needed (e.g., on page load or specific event)
return render()
