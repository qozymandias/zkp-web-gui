#![feature(associated_type_defaults)]
#![allow(clippy::type_complexity)]

use dioxus::prelude::*;
use once_cell::sync::Lazy;
use views::Dashboard;
use views::ImageDetails;
use views::Navbar;
use views::NodeDetails;
use views::TaskDetails;
use views::UserDetails;
use zkp_service_helper::helper::ZkWasmServiceHelper;

mod components;
mod utils;
mod views;

pub static ZKH: Lazy<ZkWasmServiceHelper> =
    Lazy::new(|| ZkWasmServiceHelper::new(utils::config::CONFIG.api.url.clone()));

const GLOBAL_PADDING: &str = "padding: 0rem 7rem;";

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Dashboard {},
    #[route("/task/:id")]
    TaskDetails { id: String },
    #[route("/image/:id")]
    ImageDetails { id: String },
    #[route("/node/:id")]
    NodeDetails { id: String },
    #[route("/user/:id")]
    UserDetails { id: String },
}

#[component]
fn App() -> Element {
    tracing::info!("ZKP Web App started!");
    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        document::Stylesheet { href: asset!("/assets/styling/main.css") }
        Router::<Route> {}
    }
}

fn main() {
    console_error_panic_hook::set_once();
    dioxus::launch(App);
}
