use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("./assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    let mut add_image = use_signal(|| false);
    let mut add_prove = use_signal(|| false);
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        div { id: "navbar",
            Link { id: "button", to: Route::Dashboard {}, "Home" }
            div { style: "margin-left: auto; width: fit-content;",
                button { id: "nav-button", onclick: move |_| add_image.set(true),
                    "Create New Application"
                }
                button { id: "nav-button", onclick: move |_| add_prove.set(true), "Submit Prove Task" }
            }
        }
        if add_image() {
            div { id: "popup-overlay",
                div { id: "popup-content",
                    h2 { "About" }
                    p { "This is a test navbar with modals." }
                    button { onclick: move |_| add_image.set(false), "Close" }
                }
            }
        }
        if add_prove() {
            div { id: "popup-overlay",
                div { id: "popup-content",
                    h2 { "Help" }
                    p { "Here's some help text." }
                    button { onclick: move |_| add_prove.set(false), "Close" }
                }
            }
        }
        Outlet::<Route> {}
    }
}
