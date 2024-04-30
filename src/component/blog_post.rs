use crate::model::blog_post::Post;
use leptos::*;

#[component]
pub fn BlogPost(post: Post) -> impl IntoView {
    let dt = format!("{}", post.dt.format("%B, %e, %Y %I:%M%P"));

    view! {
      <div class="block p-10">
        <div class="text-xl">{dt}</div>
        <img
          src=&post.image_url
          alt="Post thumbnail"
          class="w-96 h-32 rounded-lg object-cover my-10"
        />
        <div class="text-4xl pb-4">{&post.title}</div>
        <div>{&post.text}</div>
      </div>
    }
}
