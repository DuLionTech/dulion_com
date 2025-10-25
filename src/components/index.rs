use crate::api::fetch_section;
use leptos::prelude::*;
use leptos_router::components::A;
use crate::components::errors::errors;

#[component]
pub fn IndexDrawer(section: &'static str) -> impl IntoView {
    let section = LocalResource::new(move || async move {
        fetch_section(section).await
    });
    view! {
        <div class="drawer-side">
            <ul class="menu bg-base-200 text-base-content min-h-full w-60 p-4">
                <Transition fallback=loading>
                    <ErrorBoundary fallback=errors>{
                        move || Suspend::new(async move {
                            section.await.map(|index|
                                index.links.into_iter().map(|link| view! {
                                    <li><A href=link.href>{link.title}</A></li>
                                }).collect_view())
                            })
                        }</ErrorBoundary>
                </Transition>
            </ul>
        </div>
    }
}

fn loading() -> impl IntoView {
    view! { <li>"Loading..."</li> }
}
