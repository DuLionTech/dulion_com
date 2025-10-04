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
            <div class="card-body">
                <Transition fallback=move || view! { <p>"Loading..."</p> }>
                    <Show when=move || {
                            article_data.read().as_ref().map(Option::is_none).unwrap_or(false)
                        }>> <p>"Error loading article."</p>
                    </Show>
                    <pre>{ move || article_data.get() }</pre>
                </Transition>
            </div>
        </div>
    }
}
