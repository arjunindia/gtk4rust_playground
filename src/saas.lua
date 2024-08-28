local imageUrl = "https://picsum.photos/600"
local imagePicture = nil

local render = function()
    return vertical(
        horizontal(
            { width = 100, height = 10, halign = "center", balanced = true },
            text({ vexpand = true }, "SaaSPro"),
            horizontal(
                { hexpand = true, halign = "end" },
                link({ url = "#features" }, "Features"),
                link({ url = "#pricing" }, "Pricing"),
                link({ url = "#contact" }, "Contact"),
                button({ onclick = function() print("Sign Up clicked!") end }, "Sign Up")
            )
        ),
        vertical(
            { width = 80, marginTop = 50, marginBottom = 50 },
            heading("Revolutionize Your Workflow with SaaSPro"),
            text(
                "SaaSPro is the all-in-one solution for modern businesses. Boost productivity, streamline collaboration, and drive growth."),
            button({
                onclick = function() print("Get Started clicked!") end,
                width = 20,
                height = 10,
                marginTop = 20
            }, "Get Started"),
            image({
                ref = function(ref) imagePicture = ref end,
                width = 80,
                height = 50,
                marginTop = 30,
                marginBottom = 10
            }, imageUrl)
        ),
        vertical(
            { width = 100, id = "features" },
            heading("Key Features"),
            horizontal(
                { balanced = true },
                vertical(
                    { width = 30 },
                    heading("Seamless Integration"),
                    text("Easily connect with your existing tools and workflows.")
                ),
                vertical(
                    { width = 30 },
                    heading("Real-time Collaboration"),
                    text("Work together effortlessly with your team, anytime, anywhere.")
                ),
                vertical(
                    { width = 30 },
                    heading("Advanced Analytics"),
                    text("Gain valuable insights to make data-driven decisions.")
                )
            )
        ),
        vertical(
            { width = 100, id = "pricing", marginTop = 50 },
            heading("Flexible Pricing Plans"),
            horizontal(
                { balanced = true },
                vertical(
                    { width = 30 },
                    heading("Starter"),
                    text("$19/month"),
                    text("Perfect for small teams"),
                    button({ onclick = function() print("Choose Starter") end }, "Choose Plan")
                ),
                vertical(
                    { width = 30 },
                    heading("Professional"),
                    text("$49/month"),
                    text("Ideal for growing businesses"),
                    button({ onclick = function() print("Choose Professional") end }, "Choose Plan")
                ),
                vertical(
                    { width = 30 },
                    heading("Enterprise"),
                    text("Custom Pricing"),
                    text("Tailored solutions for large organizations"),
                    button({ onclick = function() print("Contact Sales") end }, "Contact Sales")
                )
            )
        ),
        vertical(
            { width = 100, id = "contact", marginTop = 50, marginBottom = 50 },
            heading("Get in Touch"),
            text("Have questions? We're here to help!"),
            input({ placeholder = "Your Email", width = 50 }),
            input({ placeholder = "Your Message", width = 50, height = 20 }),
            button({
                onclick = function() print("Send Message clicked!") end,
                width = 20,
                height = 10,
                marginTop = 20
            }, "Send Message")
        )
    )
end

return render()
