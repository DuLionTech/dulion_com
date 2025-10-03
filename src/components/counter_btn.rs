use leptos::prelude::*;

#[component]
pub fn Button(#[prop(default = 1)] increment: i32) -> impl IntoView {
    let (count, set_count) = signal(0);
    view! {
        <button class="btn bg-base-300" on:click=move |_| {
            set_count.set(count.get() + increment)
        }>"Click: " {count}</button>
    }
}
