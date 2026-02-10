use leptos::prelude::*;
use leptos_mview::mview;

#[component]
fn Anchor(to: &'static str) -> impl IntoView {
    mview! {
        a href=f["#{to}"] ( {to} )
    }
}

#[component]
pub fn Content() -> impl IntoView {
    mview! {
        body class="h-dvh w-dvw flex flex-row overflow-x-scroll snap-x snap-mandatory bg-base-a text-base-e font-serif" (
            div #summary class="h-full py-[50vh] w-1/2 shrink-0 snap-center ml-[50%] overflow-y-scroll overscroll-y-contain" (
                p ("Lorem ipsum dolor sit amet, consectetur " Anchor to="pharetra"; " elit.")
                p ("Fusce a libero " Anchor to="nullam"; ".")
            )
            div #exposition class="h-full py-[50vh] w-1/2 shrink-0 snap-center overflow-y-scroll overscroll-y-contain" (
                section id="pharetra" class="my-3 target:bg-base-b rounded-1 p-1" (
                    p ("Pharetra ex gravida congue. Pellentesque ornare lacus quis quam " Anchor to="saggitis"; " id aliquam urna.")
                    p ("Nulla ligula dui, consequat vel dolor vitae, luctus porta lacus. Suspendisse viverra dapibus neque. Curabitur faucibus facilisis tincidunt.")
                )
                section id="nullam" class="my-3 target:bg-base-b rounded-1 p-1" (
                    p ("Nullam vestibulum eleifend ligula, sed finibus enim.")
                    p ("Maecenas fermentum, odio in consequat ultricies, nunc neque luctus arcu, at egestas eros ex sed libero.")
                )
            )
            div #details class="h-full py-[50vh] w-1/2 shrink-0 snap-center mr-[50%] overflow-y-scroll overscroll-y-contain" (
                aside id="saggitis" class="my-3 target:bg-base-b rounded-1 p-1" (
                    p ("Sagittis ac urna ut pellentesque. Donec quis pharetra mauris. Cras rhoncus vestibulum nibh luctus luctus. Nullam gravida ex vitae turpis consequat facilisis. Cras volutpat felis vitae mauris egestas, in tempus ligula tristique. Duis lobortis risus in nisl interdum pretium.")
                    p ("Etiam sed elit at ligula interdum venenatis in a nisl. Vestibulum congue nunc ac mi rhoncus tincidunt. Nunc vel scelerisque velit. Cras sit amet sapien ultricies, commodo arcu sit amet, aliquam ex.")
                    p ("Sed dolor libero, pretium ut metus eu, tincidunt pulvinar magna. Aliquam a tortor ex. Praesent quis nisi id ante pellentesque aliquam. Fusce id porta nulla, ut facilisis neque. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae")
                )
            )
        )
    }
}
