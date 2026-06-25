use leptos::prelude::*;

#[component]
pub fn TermBox(hlt: &'static str, #[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <div class="bg-black rounded-sm border shadow-md border-black/25 p-nr min-w-xl shadow-grey inset-shadow-sm inset-shadow-white/20">
            <div
                class="overflow-x-scroll w-full font-mono text-sm/4.5 text-dim-white my-rounded-correct"
                class=("text-xs/3.75", tiny)
                inner_html=hlt
            />
        </div>
    }
}

#[component]
pub fn CodeBox(raw: &'static str, code: &'static str) -> impl IntoView {
    view! {
        <div class="bg-black rounded-sm border shadow-md border-black/25 p-nr min-w-xl shadow-grey inset-shadow-sm inset-shadow-white/20">
            <div class="sr-only">{raw}</div>
            <div
                class="overflow-x-scroll w-full font-mono text-sm whitespace-pre text-dim-white my-rounded-correct"
                inner_html=code
            />
        </div>
    }
}

#[component]
pub fn Code(raw: &'static str, code: &'static str) -> impl IntoView {
    view! {
        <div class="flex justify-center bg-black border border-y-grey py-nr not-last:mb-fr">
            <div class="sr-only">{raw}</div>
            <div
                class="font-mono text-sm whitespace-pre text-dim-white w-xl"
                inner_html=code
            />
        </div>
    }
}

#[component]
pub fn Term(hlt: &'static str, #[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <div class="flex justify-center bg-black border border-y-grey py-nr not-last:mb-fr">
            <div
                class="font-mono whitespace-pre text-sm/4.5 text-dim-white min-w-xl"
                class=("text-xs/3.75", tiny)
                inner_html=hlt
            />
        </div>
    }
}

#[component]
pub fn SourceBox(hlt: &'static str, raw: &'static str) -> impl IntoView {
    view! {
        <div class="justify-self-center bg-black rounded-sm border shadow-md @container border-black/25 p-nr min-w-xl max-w-screen shadow-grey inset-shadow-sm inset-shadow-white/20">
            <div class="sr-only">{raw}</div>
            <div
                class="overflow-x-scroll font-mono whitespace-pre text-sm/4.5 @2xl:text-xs/3.75 text-dim-white my-rounded-correct"
                inner_html=hlt
            />
        </div>
    }
}

#[component]
pub fn Source(raw: &'static str, code: &'static str) -> impl IntoView {
    view! {
        <div class="w-screen bg-black border border-y-grey py-nr not-last:mb-fr">
            <div class="sr-only">{raw}</div>
            <div
                class="mx-auto font-mono whitespace-pre text-sm/4.5 text-dim-white min-w-xl"
                inner_html=code
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
            class="font-mono mr-Znr px-xnr rounded-xs bg-dim-grey text-yellow border-dim-yellow"
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

fn _old_code(c: &'static str) -> impl IntoView {
    let big_ascenders = "bdfhkl".chars().any(|s| c.contains(s));
    let small_ascenders = !big_ascenders && "tij".chars().any(|s| c.contains(s));

    let big_descenders = "gyj".chars().any(|s| c.contains(s));
    let small_descenders = !big_descenders && "pq".chars().any(|s| c.contains(s));
    let no_descenders = !big_descenders && !small_descenders;

    view! {
        <span
            class="mr-px font-mono px-xnr rounded-xs bg-dim-grey text-yellow"
            class:pt-Znr=small_ascenders
            class:pt-znr=big_ascenders
            class:pb-Znr=no_descenders
            class:pb-xnr=big_descenders
            class:pb-znr=small_descenders
        >
            {c}
        </span>
    }
}

#[component]
pub fn Break() -> impl IntoView {
    view! {
        <span class="justify-self-stretch border-t h-[0.5px] border-mid-black p-lh m-lx" />
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
        <div class="relative self-center border shadow-md bg-[rgba(250,250,250,0.7)] min-w-xl aspect-4/3 border-t-[2em] border-black/50 rounded-2.5 shadow-black/50 before:absolute before:block before:-top-[1.25em] before:left-[1em] before:w-[0.5em] before:h-[0.5em] before:rounded-[50%] before:bg-red before:[box-shadow:0_0_0_2px_var(--color-red),1.5em_0_0_2px_var(--color-yellow),3em_0_0_2px_var(--color-green)]">
            {children()}
        </div>
    }
}
