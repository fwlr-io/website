use crate::codeblock;
use crate::ux::FancyBrowser;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <body class="flex flex-col text-base bg-white py-3g gap-3g">
            <FancyBrowser>
                <p>"Foo"</p>
            </FancyBrowser>
            <codeblock::Test />
            <codeblock::FancyTest />
            <codeblock::Code />
            <codeblock::FancyCode />
        </body>
    }
}
