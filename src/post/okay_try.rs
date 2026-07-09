use crate::block::okay_try::{self as block};
use crate::custom_block::okay_try::{self as custom};
use crate::ux::*;
use leptos::prelude::*;

#[component]
pub fn OkayTry() -> impl IntoView {
    view! {
        <Heading>"Okay Try"</Heading>
        <div class="grid grid-cols-subgrid">
            <em class="text-lg">
                r#"
                All happy "#{mono("Result")}r#"s are alike; each unhappy "#
                {mono("Some(Err)")}r#" is unhappy in its own "#
                {mono("Ok(None)")}r#" way."#
            </em>
            <p class="justify-self-end">
                <a href="https://www.gutenberg.org/files/1399/1399-h/1399-h.htm#chap01">
                    "Leo Tolstoy"
                </a>
                r#", Rust developer "#
            </p>
        </div>

        <p>
            r#"
            Every Rust codebase I've worked in has had a materially different convention
            for handling "# {mono("Result")} r#" and "# {mono("Option")} r#".
            "#
        </p>
        <p>
            r#"
            Some hew religiously to "# {mono("Result")} r#". In these projects,
            every trivial helper function's return signature is "# {code("-> io::Result<..>")}
            r#", and there is often a gulag to re-educate dozens of Rust builtins that
            dared to return an "# {mono("Option")} r#":
            "#
        </p>
        <block::HelperFunction />
        <p>
            r#"
            I've seen others that swear fealty to "# {mono("Option")} r#",
            with various safeguards against swallowing errors as None - often by
            projectile-vomiting every error to the console, in real time,
            with variations on this theme:
            "#
        </p>
        <block::ReadToOption />
        <p>
            r#"I've also seen a few that choose a third, secret way:
            white-knuckling it with nothing but "# {code("unwrap()")} r#".
            "#
        </p>
        <p>
            r#"
            While each of these conventions are functional enough,
            it seems like what they're really crying out for
            is a way to use "# {code("?")}r#" that coerces "#
            <em>"the thing they used it on"</em> r#", instead of contorting "#
            <em>"the rest of their codebase"</em>r#".
            "#
        </p>

        <Break/>

        <p>
            r#"
            "#{mono("Result")}r#"s inside "#{mono("Result")}
            r#"-returning functions,
            and "#{mono("Option")}r#"s inside "#{mono("Option")}
            r#"-returning functions,
            both work with "#{code("?")}r#":
            "#
        </p>
        <custom::QuestionMarkCorrect />

        <p>
            r#"
            But put an "#{mono("Option")}r#" in a "#{mono("Result")}
            r#"-returning function, or vice versa,
            and it won't work:
            "#
        </p>
        <custom::QuestionMarkWrongParents />

        <p>
            r#"
            The compiler keeps talking about these "#{code(".ok")}
            r#" methods, maybe that's what we want?
            "#
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
            </a> r#" of function.
            "#
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
            r#"
            But "#<em>"I know"</em>r#" what it should be. Every time I've
            gotten a "#{mono("None")}r#" when I wasn't expecting it, I've wanted
            to know where in the code it was happening. What if that was our
            general solution for the extra information needed to turn an "#
            {mono("Option")} r#" into a "#{mono("Result")}r#"?
            "#
        </p>

        <SubHeading>"Reach For The "{mono("try")}</SubHeading>
        <p>
            r#"
            Conceptually, what we want to do is implement a new method on "#
            {mono("Option")}r#". Of course, in Rust, we can't just
            monkey-patch the built-ins directly:
            "#
        </p>
        <custom::InherentImpl />
        <p>
            r#"
            Instead, we have to monkey-patch the built-ins "# <em>"indirectly"</em> r#"
            by first creating a new trait to hold our new method, and then implementing
            that trait for "# {mono("Option")}r#":
            "#
        </p>
        <block::ImplOptionExt />
        <p>
        r#"
            Finally, we need to generally and automatically get the location,
            in the source code, for where the "# {code("None")} r#" was encountered.
            Rust has a little piece of magic called "# {code("track_caller")} r#"
            that does exactly that:
            :
        "#
        </p>
        <block::TrackCaller />
        <block::NoneError />
        <block::UsefulErrorDebug />
        <block::UsefulErrorDisplay />
        <block::OkIntoResult />

        <SubHeading>"The "{mono("try")}"'s The Limit"</SubHeading>
    }
}
