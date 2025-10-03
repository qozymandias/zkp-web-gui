use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::js_sys;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

use crate::utils::web3_subscriber::js_sys::Promise;
use crate::utils::shorten_address;

#[derive(Clone, PartialEq)]
pub struct WalletAccount(String);

/// When you install MetaMask, it injects a JS object into every webpage "window.ethereum" object.
/// That object is an EIP-1193 provider with a single generic method "ethereum.request({ method: "eth_requestAccounts" })".
/// This method takes a JSON-RPC request and returns a Promise. MetaMask listens for those calls and opens the wallet popup.
/// So if you can call window.ethereum.request, you can connect, get accounts, switch networks, sign messages, etc.
///
pub async fn connect_wallet() -> Result<WalletAccount, JsValue> {
    // Get window.ethereum
    let window = window().ok_or("no window")?;
    let ethereum = js_sys::Reflect::get(&window, &JsValue::from_str("ethereum"))?;

    // Construct ethereum.request object
    let request_args = js_sys::Object::new();
    js_sys::Reflect::set(
        &request_args,
        &JsValue::from_str("method"),
        &JsValue::from_str("eth_requestAccounts"),
    )?;

    // Send request
    let request_fn = js_sys::Reflect::get(&ethereum, &JsValue::from_str("request"))?.dyn_into::<js_sys::Function>()?;
    let promise: Promise = request_fn.call1(&ethereum, &request_args)?.into();
    let result = JsFuture::from(promise).await?;

    // Map output
    let accounts: js_sys::Array = result.into();
    let account = accounts
        .iter()
        .filter_map(|it| it.as_string())
        .collect::<Vec<_>>()
        .first()
        .cloned()
        .ok_or("No accounts available")?;
    Ok(WalletAccount(account))
}

#[component]
pub fn ConnectWallet(account: Signal<Option<WalletAccount>>) -> Element {
    rsx! {
        button {
            id: "nav-button",
            onclick: move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    account.set(connect_wallet().await.inspect_err(|e| tracing::error!("{e:?}")).ok());
                });
            },
            if let Some(acc) = account() {
                "Connected - {shorten_address(&acc.0)}"
            } else {
                "Connect MetaMask"
            }
        }
    }
}
