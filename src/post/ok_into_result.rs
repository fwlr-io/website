use crate::block::ok_into_result::{self as block};
use crate::custom_block::okay_try::{self as custom};
use crate::ux::*;
use leptos::prelude::*;

#[component]
pub fn OkIntoResult() -> impl IntoView {
    view! {
        <Heading>"Ok Into Result"</Heading>
        <div class="grid grid-cols-subgrid">
            <em class="text-lg">
                r#"
                All happy "#{mono("Result")}r#"s are alike; each unhappy "#
                {mono("Some(Err)")}r#" is unhappy in its own "#
                {mono("Ok(None)")}r#" way."#
            </em>
            <p class="justify-self-end">
                r#" — Rust developer "#
                <a href="https://www.gutenberg.org/files/1399/1399-h/1399-h.htm#chap01">
                    "Leo Tolstoy"
                </a>
            </p>
        </div>

        <p>
            r#"
            Every Rust codebase I've worked in has had a materially different convention
            for handling "# {mono("Result")} r#" and "# {mono("Option")} r#"."#
        </p>
        <p>
            r#"
            I've seen projects where every trivial helper
            function's return signature is "# {code("-> io::Result<..>")}
            r#", complete with a gulag to re-educate dozens of
            Rust builtins that dared to return an "# {mono("Option")} r#":
            "#
        </p>
        <block::HelperFunction />
        <p>
            r#"
            I've seen others that swear fealty to "# {mono("Option")} r#",
            mostly by swallowing errors as None - bar one, that chose
            projectile-vomiting every error to the console, in real time,
            with variations on this theme:
            "#
        </p>
        <block::ReadToOption />
        <p>
            r#"I've also seen one that simply white-knuckled it with nothing but "#
            {code("unwrap()")} "."
        </p>
        <p>
            r#"
            While all of these codebases would have been well-served
            by better error handling, they all did "sort of" handle errors;
            it seems like what they're really crying out for
            is a way to use "# {code("?")}r#" that coerces "#
            <em>"the thing they used it on"</em> r#", instead of contorting "#
            <em>"the rest of their codebase."</em>
        </p>
        <p>r#"Can we find something like that?"#</p>

        <SubHeading>"Watch The "{mono("try")}"s"</SubHeading>

        <p>
            {mono("Result")}r#"s inside "#{mono("Result")}
            r#"-returning functions,
            and "#{mono("Option")}r#"s inside "#{mono("Option")}
            r#"-returning functions,
            both work with "#{code("?")}r#":"#
        </p>
        <custom::QuestionMarkCorrect />

        <p>
            r#"But put an "#{mono("Option")}r#" in a "#{mono("Result")}
            r#"-returning function, or vice versa,
            and it won't work:"#
        </p>
        <custom::QuestionMarkWrongParents />

        <p>
            r#"The compiler keeps talking about these "#{code(".ok")}
            r#" methods, maybe that's what we want?"#
        </p>
        <custom::QuestionMarkOk />
        <custom::QuestionMarkOkOr />

        <p>
            r#"
            Close, but we still have to look at the function's return signature
            to know whether to call "#{code(".ok")}r#" or "#{code(".ok_or")}r#".
            It would be nice if we had a symmetrical "#{code(".ok")}
            r#" method, that we could insert whenever the compiler complains
            about "#{code("?")}r#" being used in the "#
            <a href="https://journal.stuffwithstuff.com/2015/02/01/what-color-is-your-function/">
                "wrong color"
            </a> r#" of function."#
        </p>
        <p>
            r#"
            The reason we "#<em>"don't"</em>r#" have a symmetrical "#
            {code(".ok")}
            r#" method already is because the compiler can easily go
            from "#{mono("Result")}r#" to "#{mono("Option")}r#" by
            throwing away information, but going from "#{mono("Option")}
            r#" to "#{mono("Result")}r#" would require adding information,
            and there's no good way for Rust to know what that should be.
            "#
        </p>
        <p>
            r#"But "#<em>"I know"</em>r#" what it should be. Every time I've
            gotten a "#{mono("None")}r#" when I wasn't expecting it, I've wanted
            to know where in the code it was happening. What if that was our
            general solution for the extra information needed to turn an "#
            {mono("Option")} r#" into a "#{mono("Result")}r#"?"#
        </p>
        <p>r#"Can we make that? We can try."#</p>

        <SubHeading>"Reach For The "{mono("try")}</SubHeading>

        <p>
            r#"
            Conceptually, what we want to do is add a new method to "#
            {mono("Option")}r#". Of course, in Rust, you can't just
            monkey-patch the built-ins:"#
        </p>
        <custom::InherentImpl />

        <block::ImplOptionExt />
        <block::UselessError />
        <block::OptionExtTrackCaller />
        <block::NoneError />
        <block::UsefulErrorDebug />
        <block::UsefulErrorDisplay />
        <block::OkIntoResult />

        <SubHeading>"The "{mono("try")}"'s The Limit"</SubHeading>
    }
}
