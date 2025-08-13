use dioxus::prelude::*;

#[component]
pub fn Card(header: String, body: Element, header_class: Option<String>, body_class: Option<String>) -> Element {
    let h_cls = format!("card-header {}", header_class.unwrap_or_default());
    let b_cls = format!("card-body {}", body_class.unwrap_or_default());
    rsx! {
        div {
            class: "card",
            h1 {
                class: "{h_cls}",
                {header}
            }
            div {
                class: "{b_cls}",
                {body}
            }
        }
    }
}

pub trait EntryT {
    fn into_cell(self) -> Element;
}

pub trait EntryListT {
    type T: EntryT;
    fn title(&self) -> String;
    fn entries(&self) -> Vec<(&str, Self::T)>;
}

#[component]
pub fn EntryListCard<U: EntryListT + PartialEq + Clone + 'static>(
    data: U,
    lcol_class: Option<String>,
    rcol_class: Option<String>,
) -> Element {
    let title = data.title();
    let entries = data.entries();
    let lcol_cls = lcol_class.unwrap_or_default();
    let rcol_cls = rcol_class.unwrap_or_default();

    rsx! {
        div {
            class: "flex-row;",
            div {
                class: "flex-1",
                Card {
                    header: "{title}",
                    header_class: "light-blue",
                    body: rsx! {
                        div {
                            { entries
                                .into_iter()
                                .map(|(label, value)| {
                                    rsx!{
                                        div {
                                            class: "detailed-entry",
                                            div {
                                                class: "{lcol_cls}",
                                                "{label}"
                                            }
                                            div {
                                                class: "{rcol_cls}",
                                                { value.into_cell() }
                                            }
                                        }

                                    }
                                })
                            }
                        }
                    },
                }
            }
        }
    }
}
