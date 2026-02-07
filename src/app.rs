use leptos::prelude::*;
use leptos_mview::mview;

use crate::content::*;

#[component]
pub fn App() -> impl IntoView {
    let pharetra = "pharetra";
    let nullam = "nullam";
    let saggitis = "saggitis";

    mview! {
        body class="text-src-300 bg-src-900 flex h-dvh w-dvw flex-row overflow-x-scroll font-serif" (
            Summary (
                p("Lorem ipsum dolor sit amet, consectetur " Anchor text={pharetra}; " elit.")
                p("Fusce a libero " Anchor text={nullam}; ".")
            )
            Exposition (
                Section anchor={pharetra} (
                    p("Pharetra ex gravida congue. Pellentesque ornare lacus quis quam " Anchor text={saggitis}; " id aliquam urna.")
                    p("Nulla ligula dui, consequat vel dolor vitae, luctus porta lacus. Suspendisse viverra dapibus neque. Curabitur faucibus facilisis tincidunt.")
                )
                Section anchor={nullam} (
                    p("Nullam vestibulum eleifend ligula, sed finibus enim.")
                    p("Maecenas fermentum, odio in consequat ultricies, nunc neque luctus arcu, at egestas eros ex sed libero.")
                )
            )
            Details (
                Aside anchor={saggitis} (
                    p("Sagittis ac urna ut pellentesque. Donec quis pharetra mauris. Cras rhoncus vestibulum nibh luctus luctus. Nullam gravida ex vitae turpis consequat facilisis. Cras volutpat felis vitae mauris egestas, in tempus ligula tristique. Duis lobortis risus in nisl interdum pretium.")
                    p("Etiam sed elit at ligula interdum venenatis in a nisl. Vestibulum congue nunc ac mi rhoncus tincidunt. Nunc vel scelerisque velit. Cras sit amet sapien ultricies, commodo arcu sit amet, aliquam ex.")
                    p("Sed dolor libero, pretium ut metus eu, tincidunt pulvinar magna. Aliquam a tortor ex. Praesent quis nisi id ante pellentesque aliquam. Fusce id porta nulla, ut facilisis neque. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae")
                )
            )
        )
    }
}
