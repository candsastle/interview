#![allow(non_snake_case)]
use dioxus::prelude::*;
static CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
fn main() {
    dioxus::launch(App);
}
#[component]
fn DogApp(breed: String) -> Element {
    rsx! { "Breed: {breed}" }
}
#[component]
fn DisplayList(list: Vec<String>) -> Element {
    rsx! {
        ul {
            {list.iter().map(|elem| rsx! {
                li { "{elem}" }
            })}
        }
    }
}
#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        div { class: "bg-red-100",
            button { onclick: move |_| info!("Clicked!"), "Click me!" }
        }
        DogApp { breed: "Corgi" }
        div { id: "title",
            h1 { "HotDiggedyDog! 🌭" }
        }
        div { id: "dogview",
            img { src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg" }
        }
        div { id: "buttons",
            button { id: "skip", "skip!" }
            button { id: "save", "save!" }
        }
    }
}
#[component]
pub fn Hero() -> Element {
    rsx! {
        div { id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.7/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                    "💫 VSCode Extension"
                }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}
