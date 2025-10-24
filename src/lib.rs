use crate::components::avatar::Avatar;
use crate::components::menu::{Horizontal, Dropdown};
use crate::pages::reader::{ProjectsReader, ResourcesReader};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
use crate::components::article::{ProjectArticle, ResourceArticle};

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
            <div class="flex-1 grid lg:justify-center">
              <h1 class="text-3xl lg:text-6xl font-[religions]">"DuLion Technology"</h1>
              <Horizontal />
            </div>
          </nav>
          <Routes fallback=|| view! { Missing }>
            <Route path=path!("/") view=Home />
            <ParentRoute path=path!("/projects") view=ProjectsReader>
              <Route path=path!("") view=ProjectArticle />
              <Route path=path!("/:article") view=ProjectArticle />
            </ParentRoute>
            <ParentRoute path=path!("/resources") view=ResourcesReader>
              <Route path=path!("") view=ResourceArticle />
              <Route path=path!("/:article") view=ResourceArticle />
            </ParentRoute>
          </Routes>
        </Router>
    }
}
