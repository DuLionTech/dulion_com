use leptos::prelude::*;
use crate::components::article::Article;

#[component]
pub fn Projects() -> impl IntoView {
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
            <div class="drawer lg:drawer-open">
              <input id="my-drawer-2" type="checkbox" class="drawer-toggle" />
              <div class="drawer-content mx-auto flex flex-wrap justify-center items-center gap-4 items-start">
                <Article />
              </div>
              <div class="drawer-side">
                <label for="my-drawer-2" aria-label="close sidebar" class="drawer-overlay"></label>
                <ul class="menu bg-base-200 text-base-content min-h-full w-60 p-4">
                  <li><a>RV Cameras</a></li>
                  <li><a>ESP32/Rust Hydronics</a></li>
                  <li><a>ESP32/Rust WS2812B LEDs</a></li>
                </ul>
              </div>
            </div>
        </ErrorBoundary>
    }
}