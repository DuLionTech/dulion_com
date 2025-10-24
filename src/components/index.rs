use crate::api::fetch_index;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn IndexDrawer(section: &'static str) -> impl IntoView {
    let index = LocalResource::new(move || async move {
        fetch_index(section).await
    });
    let fallback = move |errors: ArcRwSignal<Errors>| {
        let error_list = move || {
            errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                    .collect_view()
            })
        };
        view! {
            <div class="error">
                <h2>"error"</h2>
                <ul>{error_list}</ul>
            </div>
        }
    };

    view! {
        <div class="drawer-side">
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                <ErrorBoundary fallback>{ move || Suspend::new(async move {
                    index.await.map(|index| view! {
                        <ul class="menu bg-base-200 text-base-content min-h-full w-60 p-4">{
                            index.index.into_iter().map(|link| view! { 
                                <li><A href=link.href>{link.title}</A></li>
                            }).collect_view()
                        }</ul>
                    })
                })}</ErrorBoundary>
            </Transition>
        </div>
    }
}
