use leptos::prelude::*;
use leptos_router::components::Outlet;
use crate::components::index::IndexDrawer;

#[component]
pub fn ProjectsReader() -> impl IntoView {
    view!{
        <Reader section="projects" />
    }
}

#[component]
pub fn ResourcesReader() -> impl IntoView {
    view! {
        <Reader section="resources" />
    }
}

#[component]
fn Reader(section: &'static str) -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
          view! {
            <h1>"Uh oh! Something went wrong!"</h1>
            <p>"Errors: "</p>
            <ul>{
              move || {
                errors
                  .get()
                  .into_iter()
                  .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                  .collect_view()
            }}</ul>
          }
        }>
          <div class="drawer lg:drawer-open">
            <input id="my-drawer-2" type="checkbox" class="drawer-toggle" />
            <div class="drawer-content w-full flex flex-wrap items-start">
              <Outlet />
            </div>
            <IndexDrawer section />
          </div>
        </ErrorBoundary>
    }
}