use crate::block::bad_vibes::{self as block};
use crate::ux::*;
use leptos::prelude::*;

#[component]
pub fn BadVibes() -> impl IntoView {
    view! {
        <p>
            r#"
            Seen it before: 'Made in China'
            "#
        </p>
        <p>
            r#"
            A bad idea whose time has come
            "#
        </p>
        <p>
            r#"
            Not even sure they are programming - how many tokens does a human dev produce?
            It should have written more software by now
            "#
        </p>
        <p>
            r#"
            The Cups And Balls
            "#
        </p>
        <p>
            r#"
            Datacenters in space, because you can't toss a molotov cocktail into low earth orbit
            "#
        </p>
        <p>
            r#"
            AGI - Altman Gets his IPO - has convinced govt it is load-bearing for economy, gets bailout
            "#
        </p>
    }
}
