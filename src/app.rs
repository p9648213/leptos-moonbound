use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::component::{blog_previews::BlogPreviews, edit_post::EditPost, view_post::ViewPost};

#[component]
pub fn Navbar() -> impl IntoView {
    view! { <nav>"This is a nav bar"</nav> }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-moonbound.css"/>
        <Title text="Welcome to Leptos"/>

        <Navbar/>

        <Router>
            <Routes>
                <Route path="" view=BlogPreviews/>
                <Route path="/edit/:post_id?" view=EditPost/>
                <Route path="/view/:post_id?" view=ViewPost/>
            </Routes>
        </Router>
    }
}
