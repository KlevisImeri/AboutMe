mod components;
mod pages;

use dioxus::prelude::*;

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(server_only! {
            ServeConfig::builder()
                .incremental(
                    dioxus::server::IncrementalRendererConfig::new()
                        .static_dir(
                            std::env::current_exe()
                                .expect("server executable path")
                                .parent()
                                .expect("server executable directory")
                                .join("public"),
                        )
                        .clear_cache(false),
                )
                .enable_out_of_order_streaming()
        })
        .launch(App);
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    Resume {},
    #[route("/cooking/")]
    Cooking {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

#[server(endpoint = "static_routes")]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    Ok(vec!["/".into(), "/cooking/".into()])
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        Router::<Route> {}
    }
}

#[component]
fn Resume() -> Element {
    rsx! {
        document::Title { "Klevis Imeri" }
        pages::resume::ResumePage {}
    }
}

#[component]
fn Cooking() -> Element {
    rsx! {
        document::Title { "Cooking — Klevis Imeri" }
        pages::cooking::CookingPage {}
    }
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        document::Title { "Page not found — Klevis Imeri" }
        components::Content {
            h1 { "Page not found" }
            p { "The page you requested could not be found." }
            Link { to: Route::Resume {}, "Return to the résumé" }
        }
    }
}
