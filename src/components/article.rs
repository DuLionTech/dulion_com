use crate::api::fetch_markdown;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use crate::components::errors::errors;

#[component]
pub fn ProjectArticle() -> impl IntoView {
    view! {
        <Article section="projects" />
    }
}

#[component]
pub fn ResourceArticle() -> impl IntoView {
    view! {
        <Article section="resources" />
    }
}

#[component]
fn Article(section: &'static str) -> impl IntoView {
    let params = use_params_map();
    let article = LocalResource::new(move || async move {
        let article = params
            .read()
            .get("article")
            .unwrap_or_else(|| "title.md".to_string());
        fetch_markdown(section, article).await
    });
    view! {
        <div class="card card-border bg-base-100 shadow-lg mx-auto">
            <div class="card-body article w-full max-w-7xl">
                <Transition fallback=loading>
                    <ErrorBoundary fallback=errors>
                        { move || Suspend::new(async move {
                            article.await.map(|article| view! {
                                <div inner_html=article />
                            })
                        })}
                    </ErrorBoundary>
                </Transition>
            </div>
        </div>
    }
}

fn loading() -> impl IntoView {
    view! { <div><h1>"Loading..."</h1></div> }
}
