use crate::model::blog_post::Post;
use leptos::*;

#[component]
pub fn BlogPost(post: ReadSignal<Post>) -> impl IntoView {
    let title = move || post.with(|post| post.title.clone());
    let text = move || post.with(|post| post.text.clone());

    view! {
        <div class="block p-10">
            <div class="text-4xl pb-4">{title}</div>
            <div>{text}</div>
        </div>
    }
}
