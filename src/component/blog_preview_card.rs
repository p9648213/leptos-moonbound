use leptos::*;

use crate::model::blog_post::Post;

#[component]
pub fn BlogPreviewCard(blog_preview: Post) -> impl IntoView {
    let dt = format!("{}", blog_preview.dt.format("%b %e, %Y %I:%M%P"));

    view! {
      <a href=format!("/view/{}", blog_preview.id)>

        <div class="transform transition duration-300 hover:scale-105 hover:shadow-2xl dark:bg-gray-600 p-6 rounded-lg flex flex-col justify-center items-center">
          <div class="w-32 h-32">
            <img
              src=blog_preview.image_url
              alt="Blog Thumbnail"
              class="w-full h-full rounded-lg object-contain"
            />
          </div>
          <div class="flex-none">
            <h2 class="text-xl font-semibold mb-2 w-48 truncate">{blog_preview.title}</h2>
            <p class="dark:text-gray-200 mb-4 w-48">{blog_preview.text}</p>
            <div class="flex justify-center">
              <span class="dark:text-gray-200">{dt}</span>
            </div>
          </div>
        </div>
      </a>
    }
}
