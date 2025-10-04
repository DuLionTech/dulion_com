use crate::components::avatar::Avatar;
use crate::components::menu::{Horizontal, Dropdown};
use crate::pages::projects::Projects;
use crate::pages::resources::Resources;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

mod components;
mod pages;
mod api;

use crate::pages::home::Home;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />
        <Title text="Phillip DuLion - Technology Enthusiast" />
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <Router>
            <nav class="navbar bg-base-200 shadow-lg mb-4">
                <Avatar />
                <Dropdown />
                <div class="flex-1 grid lg:justify-center gap-2">
                    <h1 class="text-5xl">"DuLion Technology"</h1>
                    <Horizontal />
                </div>
            </nav>
            <Routes fallback=|| view! { Missing }>
                <Route path=path!("/") view=Home />
                <Route path=path!("/projects") view=Projects />
                <Route path=path!("/resources") view=Resources />
            </Routes>
        </Router>
    }
}
