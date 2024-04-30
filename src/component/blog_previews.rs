use crate::component::blog_preview_card::BlogPreviewCard;
use crate::repository::blog_repository::get_previews;
use leptos::*;

#[component]
fn BlogDescription() -> impl IntoView {
    view! {
      <div class="p-5 flex flex-col items-center">
        <div class="mb-5 h-40 w-40 shadow-xl overflow-hidden rounded-full">
          <img
            src="https://cdn.logojoy.com/wp-content/uploads/2018/05/30164225/572.png"
            alt="logo"
          />
        </div>
        <div class="p-2 text-4xl">"Moonbound"</div>
        <div class="p-2 text-xl">"A travel blog about fun places"</div>
      </div>
    }
}

#[component]
pub fn BlogPreviews() -> impl IntoView {
    let post_resource = create_resource(
        || {},
        |_| async move { get_previews(None, None, 20, 10).await },
    );

    let previews_view = move || {
        post_resource.and_then(|previews| {
            previews
                .into_iter()
                .map(|preview| {
                    view! { <BlogPreviewCard blog_preview=preview.clone()/> }
                })
                .collect_view()
        })
    };

    view! {
      <BlogDescription/>
      <div class="dark:bg-gray-800 p-8 rounded-lg flex flex-wrap">
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
          <ErrorBoundary fallback=move |_| {
              view! { <p>"Error..."</p> }
          }>
            <div class="flex gap-3">{previews_view}</div>
          </ErrorBoundary>
        </Suspense>
      </div>
    }
}
