use leptos::prelude::*;

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
            <div class="container mx-auto flex flex-wrap justify-center items-center gap-4 mt-4 items-start">
                <div class="card card-border bg-base-100 w-2xl shadow-lg">
                    <div class="card-body">
                        <h2 class="card-title">"Software Technology"</h2>
                        <p>"My career has focused around building enterprise integration \
                        solutions, including roles handling architecture, software engineering, \
                        platform engineering, team leadership, and consulting. I've accumulated \
                        more than 25-years experience and knowledge, covering a broad spectrum of \
                        design, development, open source, and networking technologies. My primary \
                        expertise has been in building scalable enterprise integration solutions, \
                        distributed services, and web-based application architectures using various \
                        open-source technologies. I have a reputation for getting projects launched \
                        from conception, researching and evolving practical designs, quickly \
                        adopting new technologies, and delivering solid, reliable, maintainable, \
                        scalable solutions."</p>
                    </div>
                </div>
                <div class="card card-border bg-base-100 w-2xl shadow-lg">
                    <div class="card-body">
                        <h2 class="card-title">"Embedded Electronics"</h2>
                        <p>"I've always had an extensive curiosity and passion for creating \
                        interesting solutions. As small and embedded electronics have become \
                        more accessible, I've studied and experimented whatever I could get my \
                        hands on. As a result, I've accumulated a significant stash of gadgets and \
                        a staggering number of projects. I plan to write about my various projects \
                        in the blogging section."</p>
                    </div>
                </div>
                <div class="card card-border bg-base-100 w-2xl shadow-lg">
                    <div class="card-body">
                        <h2 class="card-title">"Woodworking"</h2>
                        <p>"This is another creative passion of mine. Over the years, I've \
                        accumulated enough equipment to fill a small shop. As I've gotten deeper \
                        into electronics, I've started looking for ideas for merging electronics \
                        into my woodworking passion. One example is the birdhouses I've built for \
                        my family with built-in wifi cameras. I'm also exploring addressable LED \
                        lights, searching for ways of adding lights into my woodworking \
                        projects."</p>
                    </div>
                </div>
            </div>
        </ErrorBoundary>
    }
}
