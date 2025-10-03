use leptos::prelude::*;

#[component]
pub fn Avatar() -> impl IntoView {
    view! {
        <div class="avatar">
            <div class="flex-none rounded-full border-1 border-accent shadow-lg">
                <img src="/images/lion.svg" alt="DuLion" height="110" width="110" />
            </div>
        </div>
    }
}