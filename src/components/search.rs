use dioxus::prelude::*;

pub trait SearchSelectLike {
    fn onchange(&mut self, evt: Event<FormData>);

    fn options(&self) -> Vec<&str>;

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
    sel1: U,
    sel2: V,
) -> Element {
    rsx! {
        document::Stylesheet { href: asset!("./assets/styling/search.css") }
        div {
            div { id: "search-header",
                h1 { {title} }
                div {
                    select { onchange: move |evt| sel1.onchange(evt), {sel1.selector_options()} }
                    select { onchange: move |evt| sel2.onchange(evt), {sel2.selector_options()} }
                }
            }
            div { id: "search",
                input {
                    placeholder,
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
