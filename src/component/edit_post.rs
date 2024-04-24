use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Params, PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
struct EditPostParams {
    post_id: String,
}

#[component]
pub fn EditPost() -> impl IntoView {
    let params = use_params::<EditPostParams>();
    let display_params = move || params.with(|params| params.map(|params| params.post_id));
    view! { {display_params} }
}
