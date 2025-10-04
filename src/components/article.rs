use leptos::prelude::*;
use crate::api::fetch_markdown;

#[component]
pub fn Article() -> impl IntoView {
    let (article, set_article) = signal("rv_camera");
    let article_data = Resource::new(
        move || article.get(),
        move |article| async move {
            let path = format!("/markdown/projects/{}.md", article);
            fetch_markdown(&path).await
        },
    );
    view! {
        <div class="card card-border bg-base-100 w-7xl shadow-lg">
            <div class="card-body article">
                <Transition fallback=move || view! { <p>"Loading..."</p> }>
                    <Show when=move || {
                            article_data.read().as_ref().map(Option::is_none).unwrap_or(false)
                        }> <h1>"Error loading article."</h1>
                    </Show>
                    <div inner_html={ move || article_data.get() } />
                </Transition>
            </div>
        </div>
    }
}
