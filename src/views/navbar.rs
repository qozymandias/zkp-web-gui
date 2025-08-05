use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("./assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    let mut add_image = use_signal(|| false);
    let mut add_prove = use_signal(|| false);

    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            id: "navbar",
            Link {
                id: "button",
                to: Route::Home {},
                "Home"
            }
            // Link {
            //     to: Route::Blog { id: 1 },
            //     "Blog"
            // }
            button {
                onclick: move |_| add_image.set(true),
                "Create New Application"
            }
            button {
                onclick: move |_| add_prove.set(true),
                "Submit Prove Task"
            }
        }

        if add_image() {
            div { class: "popup-overlay",
                div { class: "popup-content",
                    h2 { "About" }
                    p { "This is a test navbar with modals." }
                    button {
                        onclick: move |_| add_image.set(false),
                        "Close"
                    }
                }
            }
        }

        if add_prove() {
            div { class: "popup-overlay",
                div { class: "popup-content",
                    h2 { "Help" }
                    p { "Here's some help text." }
                    button {
                        onclick: move |_| add_prove.set(false),
                        "Close"
                    }
                }
            }
        }

        // The `Outlet` component is used to render the next component inside the layout. In this case, it will render either
        // the [`Home`] or [`Blog`] component depending on the current route.
        Outlet::<Route> {}
    }
}
