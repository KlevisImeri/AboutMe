use crate::components::Content;
use dioxus::prelude::*;

#[component]
pub fn CookingPage() -> Element {
    rsx! {
        Content {
            header { class: "relative mx-auto text-center",
                div { class: "inline-block text-center",
                    h1 { class: "cooking-title mb-[0.5em] text-2xl font-bold",
                        "Cooking Delicious Food Is One Of My Hobbies"
                    }
                }
            }
            hr {}
            h4 { "These are the foods I can cook:" }
            ul {
                li { "Chicken with many spices, bread and black tea" }
                li { "Rukola Salat with Parmixan cheese, Orange, Olive, Tomatoes and can corn" }
                li {
                    "Pasta Carbonara | with butter and white cheese | "
                    del { "tomatoes sause" }
                }
                li { "Eeggs with milk, salam, back tea with lemon and white cheesee" }
                li { "Pig meat with garlic and peper." }
                li { "Patatoes with white cheese and oragano + some type of meat (sausages & above)" }
                li { "White Rise with meat, tomatoes, pepers in a one pan" }
                li { "Angus with bread and honey" }
                li { del { "Chiken in the oven" } }
            }
            h4 { "General" }
            ul {
                li { "Butter" }
                li { "Bread" }
            }
            h4 { "Snakcs I need to buy:" }
            ul {
                li { "Doughuts | Bread with cherries and chocolate" }
                li { "Banana" }
                li { "Sushi" }
                li { "Yougurt" }
            }
        }
    }
}
