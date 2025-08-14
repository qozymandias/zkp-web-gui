use dioxus::prelude::*;

#[component]
pub fn Card(
    header: String,
    body: Element,
    card_class: Option<String>,
    header_class: Option<String>,
    body_class: Option<String>,
) -> Element {
    let c_cls = format!("card {}", card_class.unwrap_or_default());
    let h_cls = format!("card-header {}", header_class.unwrap_or_default());
    let b_cls = format!("card-body {}", body_class.unwrap_or_default());
    rsx! {
        div {
            class: "{c_cls}",
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

pub trait EntryLike {
    fn into_cell(self) -> Element;
}

pub trait EntryListLike {
    type T: EntryLike;
    fn title(&self) -> String;
    fn entries(&self) -> Vec<(&str, Self::T)>;
}

#[component]
pub fn EntryListCard<U: EntryListLike + PartialEq + Clone + 'static>(
    data: U,
    header_class: Option<String>,
    card_class: Option<String>,
    lcol_class: Option<String>,
    rcol_class: Option<String>,
) -> Element {
    let title = data.title();
    let entries = data.entries();
    let h_cls = header_class.unwrap_or_default();
    let c_cls = card_class.unwrap_or_default();
    let lcol_cls = lcol_class.unwrap_or_default();
    let rcol_cls = rcol_class.unwrap_or_default();

    rsx! {
        div {
            class: "flex-row;",
            div {
                class: "flex-1",
                Card {
                    header: "{title}",
                    header_class: "{h_cls}",
                    card_class: "{c_cls}",
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
