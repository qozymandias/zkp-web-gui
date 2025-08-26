use dioxus::prelude::*;
use serde::Serialize;

pub trait QueryFunctionHandler {
    type Input: Clone + 'static = ();
    type Data: Serialize + Clone + PartialEq + 'static;

    fn init_state() -> Self::Data;

    async fn query(inp: Self::Input) -> anyhow::Result<Self::Data>;

    fn fetch_resource(inp: Self::Input) -> Signal<Self::Data> {
        let mut data = use_signal(|| Self::init_state());
        use_future(move || {
            let inp_cp = inp.clone();
            async move {
                let res = Self::query(inp_cp).await.unwrap_or_else(|e| {
                    tracing::error!("{e}");
                    Self::init_state()
                });
                data.set(res);
            }
        });
        data
    }
}
