use convert_case::{Case, Casing};
use leptos::prelude::*;

#[component]
pub fn SourceBox(hlt: &'static str) -> impl IntoView {
    view! {
        <div class="justify-self-center bg-black rounded-sm border shadow-sm max-w-screen-2nr min-w-xl-or-screen-2nr border-mid-black p-nr shadow-grey inset-shadow-sm inset-shadow-white/20">
            <div
                class="overflow-scroll font-mono whitespace-pre text-sm/4.5 text-dim-white my-rounded-correct max-w-screen"
                inner_html=hlt
            />
        </div>
    }
}

#[component]
pub fn Source(hlt: &'static str) -> impl IntoView {
    view! {
        <div class="grid w-screen bg-black border min-w-screen max-w-screen grid-cols-subgrid border-y-grey py-nr not-last:mb-fr">
            <div
                class="justify-self-center font-mono whitespace-pre text-sm/4.5 text-dim-white min-w-xl"
                inner_html=hlt
            />
        </div>
    }
}

/// big ascenders:     b d f h k l ! " $ & ' ( ) / @ ? [ ] \ ^ ` { | } <uppercase>
/// small ascenders:   i j t # % > < <numeric>
/// big descenders:    g j y | ; ,
/// small descenders:  p q $ ( ) [ ] { } _
/// neither:           a c e m n o r s u v w x z . * + - = ~

#[inline]
fn are_big_ascenders(c: char) -> bool {
    match c {
        'b' | 'd' | 'f' | 'h' | 'k' | 'l' => true,
        '!' | '"' | '$' | '&' | '\'' | '(' | ')' | '/' | '@' | '?' | '[' | ']' | '\\' | '^'
        | '`' | '{' | '|' | '}' => true,
        'A'..='Z' => true,
        _ => false,
    }
}
#[inline]
fn are_small_ascenders(c: char) -> bool {
    match c {
        'i' | 'j' | 't' => true,
        '#' | '%' | '>' | '<' => true,
        '0'..='9' => true,
        _ => false,
    }
}
#[inline]
fn are_big_descenders(c: char) -> bool {
    match c {
        'g' | 'j' | 'y' | '|' | ';' | ',' => true,
        _ => false,
    }
}
#[inline]
fn are_small_descenders(c: char) -> bool {
    match c {
        'p' | 'q' | '$' | '(' | ')' | '[' | ']' | '{' | '}' | '_' => true,
        _ => false,
    }
}

/// visual inspection indicates one extra notch of padding
/// is needed on the bottom
pub fn code(s: &'static str) -> impl IntoView {
    let big_ascenders = s.chars().any(are_big_ascenders);
    let small_ascenders = !big_ascenders && s.chars().any(are_small_ascenders);
    let no_ascenders = !big_ascenders && !small_ascenders;

    let big_descenders = s.chars().any(are_big_descenders);
    let small_descenders = !big_descenders && s.chars().any(are_small_descenders);
    let no_descenders = !big_descenders && !small_descenders;

    view! {
        <span
            class="font-mono mr-Znr px-xnr rounded-xs bg-dim-grey text-yellow"
            class:pt-none=no_ascenders
            class:pt-Znr=small_ascenders
            class:pt-znr=big_ascenders
            class:pb-Znr=no_descenders
            class:pb-znr=small_descenders
            class:pb-xnr=big_descenders
        >
            {s}
        </span>
    }
}

#[component]
pub fn Heading(s: &'static str, title: bool) -> impl IntoView {
    view! {
        <div class="flex flex-row justify-self-stretch items-center gap-nr">
            <span
                class="border-t border-mid-black mt-nr w-[7ch]"
                class:flex-1=title
                class:pb-vnr=title
                class:pb-xnr=!title
                class:flex-none=!title
            />
            <Show when=move || title>
                <h1 class="flex-none text-3xl [font-variant:small-caps]">
                    {s}
                </h1>
            </Show>
            <Show when=move || !title>
                <h2 class="flex-none text-2xl [font-variant:small-caps]">
                    {s}
                </h2>
            </Show>
            <span
                class="flex-1 border-t border-mid-black pb-vnr mt-nr"
                class:flex-3=!title
                class:pb-xnr=!title
            />

        </div>
    }
}

pub fn title(s: &'static str) -> impl IntoView {
    view! { <Heading s=s title=true /> }
}

pub fn heading(s: &'static str) -> impl IntoView {
    view! { <Heading s=s title=false /> }
}

#[component]
pub fn Break() -> impl IntoView {
    view! {
        <span class="justify-self-stretch border-t border-mid-black pb-lh-1/3 [p+&]:has-[+p]:pb-lh-1/2 mt-lh-2/3 [p+&]:has-[+p]:mt-lh-1/2" />
    }
}

#[component]
pub fn BlackBox() -> impl IntoView {
    view! { <span class="font-mono text-4xl font-bold text-mid-black">"❒"</span> }
}

/// With credit to jarthod: https://gist.github.com/jarthod/8719db9fef8deb937f4f
#[component]
pub fn Browser(children: Children) -> impl IntoView {
    view! {
        <div class="relative self-center border shadow-sm bg-[rgba(250,250,250,0.7)] min-w-xl aspect-4/3 border-t-[2em] border-black/50 rounded-2.5 shadow-black/50 before:absolute before:block before:-top-[1.25em] before:left-[1em] before:w-[0.5em] before:h-[0.5em] before:rounded-[50%] before:bg-red before:[box-shadow:0_0_0_2px_var(--color-red),1.5em_0_0_2px_var(--color-yellow),3em_0_0_2px_var(--color-green)]">
            {children()}
        </div>
    }
}
