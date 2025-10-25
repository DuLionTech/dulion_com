use leptos::prelude::*;

pub fn errors(errors: ArcRwSignal<Errors>) -> impl IntoView {
    let error_list = move || {
        errors.with(|errors| {
            errors
                .iter()
                .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                .collect::<Vec<_>>()
        })
    };
    view! {
        <div class="error">
            <h2>"Oh no! Something went wrong!"</h2>
            <ul>{error_list}</ul>
        </div>
    }
}
