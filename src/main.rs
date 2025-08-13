use dioxus::prelude::*;
use views::Dashboard;
use views::ImageDetails;
use views::Navbar;
use views::TaskDetails;

mod components;
mod config;
mod utils;
mod views;

pub static ZKH: once_cell::sync::Lazy<zkp_service_helper::helper::ZkWasmServiceHelper> =
    once_cell::sync::Lazy::new(|| zkp_service_helper::helper::ZkWasmServiceHelper::new(config::CONFIG.api.url.clone()));

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    console_error_panic_hook::set_once();
    dioxus::launch(App);
}

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
}

#[component]
fn App() -> Element {
    tracing::info!("ZKP Web App started!");
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}
