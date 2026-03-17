use leptos::prelude::*;

#[component]
pub fn TermBox(hlt: &'static str, #[prop(optional)] tiny: bool) -> impl IntoView {
    view! {
        <div class="bg-black rounded-sm border shadow-sm border-black/25 p-nr pt-vnr min-w-xl shadow-grey inset-shadow-sm inset-shadow-white/20">
            <div
                class="font-mono text-sm/4.5 text-dim-white w-full overflow-x-scroll"
                class=("text-xs/3.75", tiny)
                inner_html=hlt
            />
        </div>
    }
}

#[component]
pub fn CodeBox(raw: &'static str, code: &'static str) -> impl IntoView {
    view! {
        <div class="bg-black rounded-md border shadow-md border-black/25 p-r min-w-xl shadow-grey inset-shadow-sm inset-shadow-white/20">
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
                class="font-mono text-sm/4.5 text-dim-white min-w-xl"
                class=("text-xs/3.75", tiny)
                inner_html=hlt
            />
        </div>
    }
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

#[component]
pub fn C(c: &'static str) -> impl IntoView {
    let no_ascenders = move || {
        for ch in "bdfhkl".chars() {
            if c.contains(ch) {
                return false;
            }
        }
        return true;
    };
    let no_descenders = move || {
        for ch in "gpqyj".chars() {
            if c.contains(ch) {
                return false;
            }
        }
        return true;
    };

    view! {
        <span
            class="font-mono rounded-xs p-xvnr pt-xxvnr mr-px bg-dim-grey text-yellow"
            class=("pt-px!", no_ascenders)
            class=("pb-xxvnr!", no_descenders)
        >
            {c}
        </span>
    }
}

#[component]
pub fn Break() -> impl IntoView {
    view! {
        <div class="flex flex-row justify-center align-middle my-lh">
            <span class="text-3xl text-mid-black">"❒"</span>
        </div>

    }
}
