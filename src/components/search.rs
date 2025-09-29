use dioxus::prelude::*;

pub trait SearchSelectLike {
    fn onchange(signal: &mut Signal<Self>, evt: Event<FormData>)
    where
        Self: Sized;

    fn reset_to_default(signal: &mut Signal<Self>)
    where
        Self: Sized;

    fn options(&self) -> Vec<&str>;

    fn is_some(&self) -> bool;

    fn read(&self) -> &str;

    fn selector_options(&self) -> Element {
        rsx! {
            {
                self.options()
                    .into_iter()
                    .map(|it| {
                        rsx! {
                            option { value: "{it}", "{it}" }
                        }
                    })
            }
        }
    }
}

#[component]
pub fn Search<U: SearchSelectLike + PartialEq + Clone + 'static, V: SearchSelectLike + PartialEq + Clone + 'static>(
    title: String,
    placeholder: String,
    input_handler: Signal<Option<String>>,
    trigger_handler: Signal<bool>,
    sel1: Signal<U>,
    sel2: Signal<V>,
) -> Element {
    let mut reset = use_signal(|| false);

    use_effect(move || {
        if reset() {
            input_handler.set(None);
            trigger_handler.set(false);
            U::reset_to_default(&mut sel1);
            V::reset_to_default(&mut sel2);
        }
    });

    rsx! {
        document::Stylesheet { href: asset!("./assets/styling/search.css") }
        div {
            div { id: "search-header",
                h1 { {title} }
                div {
                    if input_handler().is_some() || sel1().is_some() || sel2().is_some() {
                        button {
                            onclick: move |_| async move {
                                reset.set(true);
                            },
                            "Reset"
                        }
                    }
                    select {
                        value: sel1().read(),
                        autocomplete: "off",
                        onchange: move |evt| U::onchange(&mut sel1, evt),
                        {sel1().selector_options()}
                    }
                    select {
                        value: sel2().read(),
                        autocomplete: "off",
                        onchange: move |evt| V::onchange(&mut sel2, evt),
                        {sel2().selector_options()}
                    }
                }
            }
            div { id: "search",
                input {
                    placeholder,
                    value: "{input_handler().unwrap_or_default()}",
                    autocomplete: "off",
                    oninput: move |event| async move {
                        input_handler.set(Some(event.value()));
                    },
                }
                button {
                    onclick: move |_| async move {
                        trigger_handler.set(true);
                    },
                    "Search"
                }
            }
        }
    }
}
