use crate::ux::*;
use leptos::prelude::*;

#[component]
pub fn TestCodeblock() -> impl IntoView {
    let code = r##"#[component]
pub fn Code(children: Children) -> impl IntoView {
    view! {
        <div>"hello, world"</div>
    }
}"##
    .to_owned();
    view! {
        <Code>
            <Line>
                <span class="text-dim-green" data-highlight="attribute">
                    "#"
                </span>
                <span class="text-grey" data-highlight="punctuation.bracket">
                    "["
                </span>
                <span class="text-dim-green" data-highlight="attribute">
                    "component"
                </span>
                <span class="text-grey" data-highlight="punctuation.bracket">
                    "]"
                </span>
            </Line>
            <Line indent=0>
                <span class="text-dim-magenta" data-highlight="keyword">
                    "pub"
                </span>
                <span class="text-dim-white" data-highlight="other">
                    " "
                </span>
                <span class="text-dim-magenta" data-highlight="keyword">
                    "fn"
                </span>
                <span class="text-dim-white" data-highlight="other">
                    " "
                </span>
                <span class="text-red" data-highlight="function">
                    "Code"
                </span>
                <span class="text-grey" data-highlight="punctuation.bracket">
                    "("
                </span>
                <span class="text-blue" data-highlight="variable">
                    "children"
                </span>
                <span class="text-grey" data-highlight="punctuation.delimiter">
                    ":"
                </span>
                <span class="text-dim-white" data-highlight="other">
                    " "
                </span>
                <span class="text-dim-blue" data-highlight="type">
                    "Children"
                </span>
                <span class="text-grey" data-highlight="punctuation.bracket">
                    ")"
                </span>
                <span class="text-dim-white" data-highlight="other">
                    " -> "
                </span>
                <span class="text-dim-magenta" data-highlight="keyword">
                    "impl"
                </span>
                <span class="text-dim-white" data-highlight="other">
                    " "
                </span>
                <span class="text-dim-blue" data-highlight="type">
                    "IntoView"
                </span>
                <span class="text-dim-white" data-highlight="other">
                    " "
                </span>
                <span class="text-grey" data-highlight="punctuation.bracket">
                    "{"
                </span>
            </Line>
            <Line indent=4>
                <span class="text-red" data-highlight="function">
                    "view"
                </span>
                <span class="text-red" data-highlight="function">
                    "!"
                </span>
                <span class="text-dim-white" data-highlight="other">
                    " "
                </span>
                <span class="text-grey" data-highlight="punctuation.bracket">
                    "{"
                </span>
            </Line>
            <Line indent=8>
                <span class="text-grey" data-highlight="tag.delimiter">
                    "<"
                </span>
                <span class="text-dim-red" data-highlight="tag">
                    "div"
                </span>
                <span class="text-grey" data-highlight="tag.delimiter">
                    ">"
                </span>
                <span class="text-dim-green" data-highlight="string">
                    "\"hello, world\""
                </span>
                <span class="text-grey" data-highlight="tag.delimiter">
                    "</"
                </span>
                <span class="text-dim-red" data-highlight="tag">
                    "div"
                </span>
                <span class="text-grey" data-highlight="tag.delimiter">
                    ">"
                </span>
            </Line>
            <Line indent=4>
                <span class="text-grey" data-highlight="punctuation.bracket">
                    "}"
                </span>
            </Line>
            <Line indent=0>
                <span class="text-grey" data-highlight="punctuation.bracket">
                    "}"
                </span>
            </Line>
            <Raw code=code />
        </Code>
    }
}
