use crate::{
    component::blog_post::BlogPost, model::blog_post::Post, repository::blog_repository::get_post,
};
use leptos::*;
use leptos_router::use_params_map;

#[component]
pub fn ViewPost() -> impl IntoView {
    let params = use_params_map();

    let post_resource = create_resource(
        move || params.with(|params| params.get("post_id").cloned()),
        |params| async move {
            match params {
                Some(id) => get_post(id).await,
                None => Ok(Post::new_empty()),
            }
        },
    );

    let post_view = move || {
        post_resource.and_then(|post| {
            let post_saved = post.clone();
            view! {
              <div class="w-full flex justify-center">
                <div class="max-w-[800]">
                  <div class="flex justify-center pt-10">
                    <a href=format!("/edit/{}", &post.id)>"Edit"</a>
                  </div>
                  <BlogPost post=post_saved/>
                </div>
              </div>
            }
        })
    };

    view! {
      <Suspense fallback=move || view! { <p>"Loading..."</p> }>
        <ErrorBoundary fallback=move |_| {
            view! { <p>"Error..."</p> }
        }>{post_view}</ErrorBoundary>
      </Suspense>
    }
}
