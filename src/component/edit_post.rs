use crate::component::blog_post::BlogPost;
use crate::model::blog_post::Post;
use leptos::{ev::Event, *};

#[component]
pub fn EditPost() -> impl IntoView {
    let (post, set_post) = create_signal(Post::new_empty());

    let on_input_title = move |event: Event| {
        set_post.update(|post| post.title = event_target_value(&event));
    };

    let on_input_text = move |event: Event| {
        set_post.update(|post| post.text = event_target_value(&event));
    };

    view! {
        <div class="flex h-screen">
            <div class="min-w-[50%] max-h-[90%] text-gray-200 dark:bg-gray-800 bg-gray-100 p-10 rounded-md">
                <form>
                    <label class="block mb-4">
                        <span>Title</span>
                        <input
                            class="mt-1 p-2 w-full bg-gray-700 text-gray-200 focus:outline-white focus:border-white rounded-md"
                            type="text"
                            id="title"
                            name="title"
                            on:input=move |event| on_input_title(event)
                            prop:value=move || post.with(|post| post.title.clone())
                        />
                    </label>
                    <label class="block mb-4">
                        <span>Entry</span>
                        <textarea
                            class="mt-1 p-2 w-full"
                            name="text"
                            id="text"
                            on:input=move |event| on_input_text(event)
                            prop:value=move || post.with(|post| post.text.clone())
                        ></textarea>
                    </label>
                </form>
            </div>
            <div>
                <BlogPost post=post/>
            </div>
        </div>
    }
}
