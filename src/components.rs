use dioxus::prelude::*;

#[component]
pub fn Content(children: Element) -> Element {
    rsx! {
        main {
            class: "content p-10 text-gray-900 leading-normal break-words transition-colors duration-200",
            {children}
        }
    }
}

#[component]
pub fn SectionHeading(title: &'static str) -> Element {
    rsx! {
        hr {}
        h4 { "[{title}]" }
    }
}

#[component]
pub fn DataTable(children: Element) -> Element {
    rsx! {
        div { class: "max-w-full overflow-x-auto",
            table { {children} }
        }
    }
}
