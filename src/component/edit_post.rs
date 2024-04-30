use crate::component::toast::{ToastMessage, ToastType};
use crate::model::blog_post::Post;
use crate::repository::blog_repository::UpsertPost;
use crate::{component::blog_post::BlogPost, repository::blog_repository::get_post};
use chrono::{Local, NaiveDateTime};
use leptos::{ev::Event, *};
use leptos_router::ActionForm;
use leptos_router::{use_navigate, use_params_map};

#[component]
pub fn EditPost() -> impl IntoView {
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

    let on_input_title = move |event: Event| {
        post_resource.update(|current_post| {
            if let Some(Ok(post)) = current_post {
                post.title = event_target_value(&event);
            }
        });
    };

    let on_input_text = move |event: Event| {
        post_resource.update(|current_post| {
            if let Some(Ok(post)) = current_post {
                post.text = event_target_value(&event);
            }
        });
    };

    let on_input_image = move |event: Event| {
        post_resource.update(|current_post| {
            if let Some(Ok(post)) = current_post {
                post.image_url = event_target_value(&event);
            }
        });
    };

    let on_input_datetime = move |event: Event| {
        let dt = event_target_value(&event);
        let chrono_dt = NaiveDateTime::parse_from_str(&dt, "%Y-%m-%dT%H:%M");
        let utc_dt = match chrono_dt {
            Ok(dt) => dt,
            _ => Local::now().naive_local(),
        };

        post_resource.update(|current_post| {
            if let Some(Ok(post)) = current_post {
                post.dt = utc_dt;
            }
        });
    };

    let upsert_post = create_server_action::<UpsertPost>();

    let set_toast: WriteSignal<ToastMessage> = expect_context();

    create_effect(move |_| {
        let id = upsert_post.value().get();
        if let Some(Ok(id)) = id {
            set_toast.set(ToastMessage {
                message: String::from("Post created"),
                toast_type: ToastType::Success,
                visible: true,
            });
            let navigate = use_navigate();
            navigate(format!("/view/{}", id).as_str(), Default::default());
        }
    });

    view! {
      <Suspense fallback=move || view! { <p>"Loading..."</p> }>
        <ErrorBoundary fallback=move |_| {
            view! { <p>"Error"</p> }
        }>
          <div class="flex h-screen">
            <div class="min-w-[50%] max-h-[90%] text-gray-200 dark:bg-gray-800 bg-gray-100 p-10 rounded-md">
              <ActionForm action=upsert_post>
                <label class="block mb-4">
                  <span>"Date"</span>
                  <input
                    class="mt-1 p-2 w-full bg-gray-700 text-gray-200 focus:outline-white focus:border-white rounded-md"
                    type="datetime-local"
                    id="datetime"
                    name="dt"
                    on:input=move |event| on_input_datetime(event)
                    prop:value=move || {
                        post_resource
                            .get()
                            .and_then(|res| {
                                res.map(|post| post.dt.format("%Y-%m-%dT%H:%M").to_string()).ok()
                            })
                    }
                  />

                </label>
                <label class="block mb-4">
                  <span>"Image URL"</span>
                  <input
                    class="mt-1 p-2 w-full bg-gray-700 text-gray-200 focus:outline-white focus:border-white rounded-md"
                    type="text"
                    id="image_url"
                    name="image_url"
                    on:input=move |event| on_input_image(event)
                    prop:value=move || {
                        post_resource.get().and_then(|res| res.map(|post| post.image_url).ok())
                    }
                  />

                </label>
                <label class="block mb-4">
                  <span>"Title"</span>
                  <input
                    class="mt-1 p-2 w-full bg-gray-700 text-gray-200 focus:outline-white focus:border-white rounded-md"
                    type="text"
                    id="title"
                    name="title"
                    on:input=move |event| on_input_title(event)
                    prop:value=move || {
                        post_resource.get().and_then(|res| res.map(|post| post.title).ok())
                    }
                  />

                </label>
                <label class="block mb-4">
                  <span>"Entry"</span>
                  <textarea
                    class="mt-1 p-2 w-full"
                    name="text"
                    id="text"
                    on:input=move |event| on_input_text(event)
                    prop:value=move || {
                        post_resource.get().and_then(|res| res.map(|post| post.text).ok())
                    }
                  >
                  </textarea>
                </label>
                <div class="flex justify-center pb-4">
                  <button
                    type="submit"
                    class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800"
                  >
                    "Submit"
                  </button>
                </div>
              </ActionForm>
            </div>
            <div>
              {move || { post_resource.and_then(|post| view! { <BlogPost post=post.clone()/> }) }}
            </div>
          </div>
        </ErrorBoundary>
      </Suspense>
    }
}
