use dioxus::prelude::*;
use zkp_service_helper::interface::AddProveTaskRestrictions;
use zkp_service_helper::interface::AppConfig;
use zkp_service_helper::interface::ConciseTask;
use zkp_service_helper::interface::Image;
use zkp_service_helper::interface::ProvePaymentSrc;
use zkp_service_helper::interface::TaskType;

use crate::components::card::Card;
use crate::components::card::EntryListCard;
use crate::components::card::EntryListLike;
use crate::components::table::Table;
use crate::utils::AddressKind;
use crate::utils::AddressStyle;
use crate::utils::TimestampStyle;
use crate::utils::ZkEntry;
use crate::GLOBAL_PADDING;
use crate::ZKH;

#[derive(Clone, PartialEq)]
struct DetailedImage {
    submit_time: String,
    networks: Vec<String>,
    creator_paid_proof: String,
    only_creator_add: String,
    proofs_submitted: u64,
    image: Image,
}

impl DetailedImage {
    fn new(
        image_in: Signal<Option<Image>>,
        config_in: Signal<Option<AppConfig>>,
        task_in: Signal<Option<ConciseTask>>,
        proofs_submitted_in: Signal<Option<u64>>,
    ) -> Option<Self> {
        let image = image_in.read().as_ref().cloned()?;
        let config = config_in.read().as_ref().cloned()?;
        let task = task_in.read().as_ref().cloned()?;
        let proofs_submitted = proofs_submitted_in.read().as_ref().cloned()?;
        let networks = config
            .chain_info_list
            .iter()
            .filter(|it| image.auto_submit_network_ids.iter().any(|id| *id == it.chain_id))
            .map(|it| it.chain_name.clone())
            .collect();
        Some(Self {
            submit_time: task.submit_time,
            networks,
            creator_paid_proof: match image.prove_payment_src {
                ProvePaymentSrc::Default => "No",
                ProvePaymentSrc::CreatorPay => "Yes",
            }
            .to_string(),
            only_creator_add: match image.add_prove_task_restrictions {
                AddProveTaskRestrictions::Anyone => "No",
                AddProveTaskRestrictions::CreatorOnly => "Yes",
            }
            .to_string(),
            proofs_submitted,
            image,
        })
    }
}

impl EntryListLike for Option<DetailedImage> {
    type T = ZkEntry;

    fn title(&self) -> String {
        "Overview".to_string()
    }

    fn entries(&self) -> Vec<(&str, ZkEntry)> {
        self.as_ref()
            .map(|it| {
                vec![
                    (
                        "Owner",
                        ZkEntry::Address(it.image.user_address.clone(), AddressStyle::Detailed, AddressKind::User),
                    ),
                    (
                        "Created On",
                        ZkEntry::Timestamp(Some(it.submit_time.clone()), TimestampStyle::Simple),
                    ),
                    ("Auto Submit Proof Network(s)", ZkEntry::Raw(it.networks.join(" "))),
                    ("Circuit Size", ZkEntry::Raw(it.image.circuit_size.to_string())),
                    ("Creator Paid Proof", ZkEntry::Raw(it.creator_paid_proof.clone())),
                    (
                        "Only image creator can add prove task",
                        ZkEntry::Raw(it.only_creator_add.clone()),
                    ),
                    ("Proofs submitted", ZkEntry::Raw(it.proofs_submitted.to_string())),
                    ("Image Commitment", ZkEntry::Checksum(it.image.checksum.clone())),
                    ("Image Status", ZkEntry::Raw(it.image.status.clone())),
                    (
                        "Shared Data Image",
                        ZkEntry::Raw(
                            it.image
                                .inherited_merkle_data_info
                                .as_ref()
                                .map(|x| x.md5.clone())
                                .unwrap_or("NA".to_string()),
                        ),
                    ),
                ]
            })
            .unwrap_or_default()
    }
}

#[component]
pub fn ImageDetails(id: String) -> Element {
    tracing::info!("Image detail loading {id}");

    // TODO: make a trait for these to impl like QueryFunctionHandler
    let md5 = id.clone();
    let mut config = use_signal(|| Option::<AppConfig>::None);
    use_future(move || async move {
        config.set(ZKH.query_config().await.inspect_err(|e| tracing::error!("{e}")).ok());
    });

    let md5_for_setup = md5.clone();
    let mut setup = use_signal(|| Option::<ConciseTask>::None);
    let mut setups = use_signal(Vec::<ConciseTask>::new);
    use_future(move || {
        let md5_cp = md5_for_setup.clone();
        async move {
            let res = ZKH
                .query_concise_tasks(None, Some(md5_cp), None, Some(TaskType::Setup), None, None, None)
                .await
                .inspect_err(|e| tracing::error!("{e}"))
                .ok()
                .map(|res| res.data);
            setup.set(res.clone().and_then(|res| res.first().cloned()));
            setups.set(res.unwrap_or(vec![]));
        }
    });

    let md5_for_proves = md5.clone();
    let mut proves = use_signal(Vec::<ConciseTask>::new);
    let mut prove_count = use_signal(|| Option::<u64>::None);
    use_future(move || {
        let md5_cp = md5_for_proves.clone();
        async move {
            let res = ZKH
                .query_concise_tasks(None, Some(md5_cp), None, Some(TaskType::Prove), None, None, None)
                .await;
            prove_count.set(res.as_ref().map(|it| it.total).ok());
            proves.set(res.map(|res| res.data).unwrap_or(vec![]));
        }
    });

    let md5_for_image = md5.clone();
    let mut image = use_signal(|| Option::<Image>::None);
    use_future(move || {
        let md5_cp = md5_for_image.clone();
        async move {
            let result = ZKH.query_image(md5_cp).await.unwrap_or_default();
            image.set(result);
        }
    });

    let desc = image.as_ref().map(|it| it.description_url.clone()).unwrap_or("NA".to_string());
    let left = format!("Image Hash {}", md5);
    let right = image.as_ref().map(|it| it.user_address.clone()).unwrap_or("NA".to_string());
    rsx! {
        div { style: GLOBAL_PADDING,
            div { id: "detail-header",
                div { "{left}" }
                div { id: "right-div", "{right}" }
            }
        }
        div { class: "stretched-nested-div-parent",
            div { class: "flex-1 pad-5",
                EntryListCard {
                    data: DetailedImage::new(image, config, setup, prove_count),
                    lcol_class: "image-details-col",
                }
            }
            div { class: "pad-5 stretched-nested-div",
                Card {
                    header: "Description",
                    body: rsx! {
                    "{desc}"
                    },
                }
            }
        }
        Table { data: proves() }
        Table { data: setups() }
    }
}
