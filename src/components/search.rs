use dioxus::prelude::*;

pub trait SearchSelectLike: Sized {
    fn raw_options() -> Vec<Self>;

    fn to_string(it: &Self) -> String;

    fn from_string(it: String) -> Self;

    fn all() -> &'static str {
        "All"
    }

    fn options() -> Vec<String> {
        [
            vec![Self::all().to_string()],
            Self::raw_options().iter().map(Self::to_string).collect(),
        ]
        .concat()
    }

    fn read(it: &Option<Self>) -> String {
        it.as_ref().map(Self::to_string).unwrap_or(Self::all().to_string())
    }
}

trait SearchSelectRender: SearchSelectLike {
    fn selector_options() -> Element {
        rsx! {
            {
                Self::options()
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

impl<T: SearchSelectLike> SearchSelectRender for T {}

trait SearchSelectSignals: SearchSelectLike {
    fn onchange(signal: &mut Signal<Option<Self>>, evt: Event<FormData>) {
        let str = evt.value();
        signal.set(if str == Self::all() {
            None
        } else {
            Some(Self::from_string(str))
        })
    }
}

impl<T: SearchSelectLike> SearchSelectSignals for T {}

#[component]
pub fn Search<
    U: SearchSelectLike + SearchSelectRender + SearchSelectSignals + PartialEq + Clone + 'static,
    V: SearchSelectLike + SearchSelectRender + SearchSelectSignals + PartialEq + Clone + 'static,
>(
    title: String,
    placeholder: String,
    input: Signal<Option<String>>,
    trigger: Signal<bool>,
    sel1: Signal<Option<U>>,
    sel2: Signal<Option<V>>,
) -> Element {
    let mut reset = use_signal(|| false);

    use_effect(move || {
        if reset() {
            input.set(None);
            trigger.set(false);
            sel1.set(None);
            sel2.set(None);
        }
    });

    rsx! {
        document::Stylesheet { href: asset!("./assets/styling/search.css") }
        div {
            div { id: "search-header",
                h1 { {title} }
                div {
                    if input().is_some() || sel1().is_some() || sel2().is_some() {
                        button {
                            onclick: move |_| async move {
                                reset.set(true);
                            },
                            "Reset"
                        }
                    }
                    select {
                        value: U::read(&sel1()),
                        autocomplete: "off",
                        onchange: move |evt| U::onchange(&mut sel1, evt),
                        {U::selector_options()}
                    }
                    select {
                        value: V::read(&sel2()),
                        autocomplete: "off",
                        onchange: move |evt| V::onchange(&mut sel2, evt),
                        {V::selector_options()}
                    }
                }
            }
            div { id: "search",
                input {
                    placeholder,
                    value: input().unwrap_or_default(),
                    autocomplete: "off",
                    oninput: move |event| async move {
                        input.set(Some(event.value()));
                    },
                }
                button {
                    onclick: move |_| async move {
                        trigger.set(true);
                    },
                    "Search"
                }
            }
        }
    }
}
