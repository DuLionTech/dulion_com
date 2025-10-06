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
            <div class="container mx-auto flex flex-wrap justify-center items-center gap-4 items-start">
                <div class="card card-border card-side bg-base-100 w-xl shadow-lg">
                    <figure class="w-1/3">
                        <img src="images/soft_tech.jpg" alt="Software Technology" />
                    </figure>
                    <div class="card-body text-base w-2/3">
                        <h2 class="card-title">"Software Technology"</h2>
                        <p>"With over 25 years of experience, my career has been primarily \
                        dedicated to building enterprise integration solutions. I've held diverse \
                        roles including architect, software engineer, platform engineer, team \
                        leader, and consultant, accumulating extensive knowledge in design, \
                        development, open source, and networking technologies. My core expertise \
                        lies in crafting scalable enterprise integrations, distributed services, \
                        and web-based application architectures leveraging open-source tools. I am \
                        known for driving projects from concept to launch, meticulously researching \
                        and evolving practical designs, rapidly adopting new technologies, and \
                        consistently delivering robust, reliable, maintainable, and scalable \
                        solutions."</p>
                    </div>
                </div>
                <div class="card card-border card-side bg-base-100 w-xl shadow-lg">
                    <figure class="w-1/3">
                        <img
                            src="images/embedded.jpg"
                            alt="Embedded Electronics" />
                    </figure>
                    <div class="card-body text-base w-2/3">
                        <h2 class="card-title">"Embedded Electronics"</h2>
                        <p>"My innate curiosity has always driven me to explore and understand how \
                        things work, fueling a deep-seated passion for developing innovative and \
                        intriguing solutions. This drive has intensified with the increasing \
                        accessibility of small and embedded electronics. I've dedicated countless \
                        hours to studying and experimenting with a wide array of components, \
                        microcontrollers, and development boards, eagerly acquiring whatever I \
                        could get my hands on."</p>
                        <p>"As a result of this continuous exploration, I've amassed a truly \
                        impressive and, at times, overwhelming collection of gadgets, electronic \
                        components, and microcontrollers. From single-board computers to sensors, \
                        actuators, cameras, and various other modules, my workbench and cabinets \
                        are testament to this ongoing passion. This extensive "stash" represents \
                        not just components, but countless hours of learning, problem-solving, and \
                        creative endeavor."</p>
                    </div>
                </div>
                <div class="card card-border card-side bg-base-100 w-xl shadow-lg">
                    <figure class="w-1/3">
                        <img
                            src="images/woodworking.jpg"
                            alt="Woodworking" />
                    </figure>
                    <div class="card-body text-base w-2/3">
                        <h2 class="card-title">"Woodworking"</h2>
                        <p>"Woodworking has become a deeply cherished creative outlet, offering a \
                        satisfying blend of meticulous craftsmanship and artistic expression. Over \
                        the years, I've cultivated a robust collection of woodworking equipment, \
                        meticulously acquired to outfit a fully functional small shop. This \
                        includes a comprehensive range of tools, from precision saws and robust \
                        planers to versatile routers and an assortment of hand tools, all carefully \
                        maintained and ready for use."</p>
                        <p>"My journey in woodworking has naturally intersected with my growing \
                        expertise in electronics. This fusion has opened up exciting new avenues \
                        for innovation and personalization in my projects. A prime example of this \
                        integration can be seen in the custom birdhouses I've crafted for family \
                        members. These aren't just ordinary birdhouses; each one thoughtfully \
                        incorporates a miniature Wi-Fi camera, allowing for real-time observation \
                        of nesting birds from a smartphone or computer. This blend of natural \
                        beauty and modern technology brings a unique dimension to backyard wildlife \
                        appreciation."</p>
                        <p>"Beyond the practical applications, I'm actively exploring the aesthetic \
                        potential of integrating electronics into my woodworking designs. For \
                        example, I'm exploring the incorporation of addressable LED lights. Imagine \
                        custom-built shelves with integrated lighting that can change color and \
                        pattern, or a handcrafted piece of furniture that subtly illuminates to \
                        create a dynamic ambiance. The possibilities for adding an interactive and \
                        visually striking element to my wooden creations are truly captivating."</p>
                    </div>
                </div>
            </div>
        </ErrorBoundary>
    }
}
