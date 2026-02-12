use leptos::prelude::*;
use leptos_mview::mview;

#[component]
pub fn Content() -> impl IntoView {
    view! {
        <body class="flex overflow-x-scroll flex-row font-serif text-black bg-white h-dvh w-dvw snap-x snap-mandatory">
            <div
                id="summary"
                class="overflow-y-scroll overscroll-y-contain w-1/2 h-full py-[50vh] shrink-0 snap-center ml-[50%]"
            >
                <p>
                    "Lorem ipsum dolor sit amet, consectetur "
                    <a href="#pharetra">"pharetra"</a> " elit."
                </p>
                <p>"Fusce a libero " <a href="#nullam">"nullam"</a> "."</p>
            </div>
            <div
                id="exposition"
                class="overflow-y-scroll overscroll-y-contain w-1/2 h-full py-[50vh] shrink-0 snap-center"
            >
                <section
                    id="pharetra"
                    class="p-1g my-3g target:bg-dim-white rounded-1g"
                >
                    <p>
                        "Pharetra ex gravida congue. Pellentesque ornare lacus quis quam "
                        <a href="#saggitis">"saggitis"</a> " id aliquam urna."
                    </p>
                    <p>
                        "Nulla ligula dui, consequat vel dolor vitae, luctus porta lacus. Suspendisse viverra dapibus neque. Curabitur faucibus facilisis tincidunt."
                    </p>
                </section>
                <section
                    id="nullam"
                    class="p-1g my-3g target:bg-dim-white rounded-1g"
                >
                    <p>
                        "Nullam vestibulum eleifend ligula, sed finibus enim."
                    </p>
                    <p>
                        "Maecenas fermentum, odio in consequat ultricies, nunc neque luctus arcu, at egestas eros ex sed libero."
                    </p>
                </section>
            </div>
            <div
                id="details"
                class="overflow-y-scroll overscroll-y-contain w-1/2 h-full py-[50vh] shrink-0 snap-center mr-[50%]"
            >
                <aside
                    id="saggitis"
                    class="p-1g my-3g target:bg-dim-white rounded-1g"
                >
                    <p>
                        "Sagittis ac urna ut pellentesque. Donec quis pharetra mauris. Cras rhoncus vestibulum nibh luctus luctus. Nullam gravida ex vitae turpis consequat facilisis. Cras volutpat felis vitae mauris egestas, in tempus ligula tristique. Duis lobortis risus in nisl interdum pretium."
                    </p>
                    <p>
                        "Etiam sed elit at ligula interdum venenatis in a nisl. Vestibulum congue nunc ac mi rhoncus tincidunt. Nunc vel scelerisque velit. Cras sit amet sapien ultricies, commodo arcu sit amet, aliquam ex."
                    </p>
                    <p>
                        "Sed dolor libero, pretium ut metus eu, tincidunt pulvinar magna. Aliquam a tortor ex. Praesent quis nisi id ante pellentesque aliquam. Fusce id porta nulla, ut facilisis neque. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae"
                    </p>
                </aside>
            </div>
        </body>
    }
}

#[component]
fn Anchor(to: &'static str) -> impl IntoView {
    mview! {
        a href=f["#{to}"] ( {to} )
    }
}

#[component]
pub fn OldContent() -> impl IntoView {
    mview! {
        body class="h-dvh w-dvw flex flex-row overflow-x-scroll snap-x snap-mandatory bg-white text-black font-serif" (
            div #summary class="h-full py-[50vh] w-1/2 shrink-0 snap-center ml-[50%] overflow-y-scroll overscroll-y-contain" (
                p ("Lorem ipsum dolor sit amet, consectetur " Anchor to="pharetra"; " elit.")
                p ("Fusce a libero " Anchor to="nullam"; ".")
            )
            div #exposition class="h-full py-[50vh] w-1/2 shrink-0 snap-center overflow-y-scroll overscroll-y-contain" (
                section id="pharetra" class="my-3g target:bg-dim-white rounded-1g p-1g" (
                    p ("Pharetra ex gravida congue. Pellentesque ornare lacus quis quam " Anchor to="saggitis"; " id aliquam urna.")
                    p ("Nulla ligula dui, consequat vel dolor vitae, luctus porta lacus. Suspendisse viverra dapibus neque. Curabitur faucibus facilisis tincidunt.")
                )
                section id="nullam" class="my-3g target:bg-dim-white rounded-1g p-1g" (
                    p ("Nullam vestibulum eleifend ligula, sed finibus enim.")
                    p ("Maecenas fermentum, odio in consequat ultricies, nunc neque luctus arcu, at egestas eros ex sed libero.")
                )
            )
            div #details class="h-full py-[50vh] w-1/2 shrink-0 snap-center mr-[50%] overflow-y-scroll overscroll-y-contain" (
                aside id="saggitis" class="my-3g target:bg-dim-white rounded-1g p-1g" (
                    p ("Sagittis ac urna ut pellentesque. Donec quis pharetra mauris. Cras rhoncus vestibulum nibh luctus luctus. Nullam gravida ex vitae turpis consequat facilisis. Cras volutpat felis vitae mauris egestas, in tempus ligula tristique. Duis lobortis risus in nisl interdum pretium.")
                    p ("Etiam sed elit at ligula interdum venenatis in a nisl. Vestibulum congue nunc ac mi rhoncus tincidunt. Nunc vel scelerisque velit. Cras sit amet sapien ultricies, commodo arcu sit amet, aliquam ex.")
                    p ("Sed dolor libero, pretium ut metus eu, tincidunt pulvinar magna. Aliquam a tortor ex. Praesent quis nisi id ante pellentesque aliquam. Fusce id porta nulla, ut facilisis neque. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae")
                )
            )
        )
    }
}
