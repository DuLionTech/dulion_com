use leptos::prelude::*;
use crate::components::counter_btn::Button;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>
                <p>"Errors: "</p>
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}
                </ul>
            }
        }>
            <div class="navbar bg-base-200 shadow-lg">
                <div class="flex-none">
                <img
                    src="images/lion.svg"
                    alt="DuLion Logo"
                    height="110"
                    width="110"
                />
                </div>
                <h1 class="flex-1 flex text-4xl justify-center">"DuLion Technology"</h1>
                <p class="flex-none"><Button /></p>
            </div>
        </ErrorBoundary>
    }
}
