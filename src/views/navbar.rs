use crate::utils::web3_subscriber::{ConnectWallet, WalletAccount};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let account = use_signal(|| Option::<WalletAccount>::None);
    let mut add_image = use_signal(|| false);
    let mut add_prove = use_signal(|| false);
    rsx! {
        document::Stylesheet { rel: "stylesheet", href: asset!("./assets/styling/navbar.css") }
        div { id: "navbar",
            Link { id: "button", to: Route::Dashboard {}, "Home" }
            div { style: "margin-left: auto; width: fit-content;",
                button { id: "nav-button", onclick: move |_| add_image.set(true),
                    "Create New Application"
                }
                button { id: "nav-button", onclick: move |_| add_prove.set(true), "Submit Prove Task" }
                ConnectWallet { account }
            }
        }
        // TODO: make these proper components
        if add_image() {
            div { class: "popup-task-overlay",
                div { class: "popup-task",
                    h2 { "About" }
                    p { "This is a test navbar with modals." }
                    button { onclick: move |_| add_image.set(false), "Close" }
                }
            }
        }
        if add_prove() {
            div { class: "popup-task-overlay",
                div { class: "popup-task",
                    h2 { "Help" }
                    p { "Here's some help text." }
                    button { onclick: move |_| add_prove.set(false), "Close" }
                }
            }
        }
        Outlet::<Route> {}
    }
}
