use dioxus::prelude::*;

pub trait QueryFunctionHandler {
    type Input: Clone + 'static = ();
    type Data: serde::Serialize + Clone + PartialEq + Default + 'static;

    async fn query(inp: Self::Input) -> anyhow::Result<Self::Data>;

    fn fetch_resource(inp: Self::Input) -> Signal<Self::Data> {
        let mut data = use_signal(Self::Data::default);
        use_future(move || {
            let inp_cp = inp.clone();
            async move {
                data.set(
                    Self::query(inp_cp)
                        .await
                        .inspect_err(|e| tracing::error!("{e}"))
                        .unwrap_or_else(|_| Self::Data::default()),
                );
            }
        });
        data
    }
}
