use crate::codeblock::TestCodeblock;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <body class="p-3 bg-white">
            <TestCodeblock />
        </body>
    }
}
