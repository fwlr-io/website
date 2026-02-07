use leptos::prelude::*;
use leptos_mview::mview;

pub fn anchor_section(text: &'static str) -> (_, _) {
    #[component]
    fn Anchor() -> impl IntoView {
        mview! {
            " "
            a href={move || format!("#{}", text) } (
                {text}
            )
            " "
        }
    }

    #[component]
    fn Section(children: Children) -> impl IntoView {
        mview! {
            section id={text} class="border-src-350 target:bg-src-800 m-4 rounded-md border p-4" (
                {children()}
            )
        }
    }

    return (Anchor, Section);
}

trait HOC {
    type C;
    fn call(&self) -> Self::C;
}

impl<F: Fn() -> R, R> HOC for F {
    type C = R;
    fn call(&self) -> Self::C {
        self()
    }
}

trait HOC1<P> {
    type C;
    fn call(&self, p: P) -> Self::C;
}

impl<F: Fn(A) -> R, A, R> HOC1<A> for F {
    type C = R;
    fn call(&self, p: A) -> Self::C {
        self(p)
    }
}

pub fn anchor_section_1(
    text: &'static str,
) -> (
    impl HOC<C = impl IntoView>,
    impl HOC1<Children, C = impl IntoView>,
) {
    #[component]
    fn Anchor() -> impl IntoView {
        mview! {
            " "
            a href={move || format!("#{}", text) } (
                {text}
            )
            " "
        }
    }

    #[component]
    fn Section(children: Children) -> impl IntoView {
        mview! {
            section id={text} class="border-src-350 target:bg-src-800 m-4 rounded-md border p-4" (
                {children()}
            )
        }
    }

    return (Anchor, Section);
}

pub fn anchor_section_2(
    text: &'static str,
) -> (
    impl HOC<C = impl IntoView>,
    impl HOC1<Children, C = impl IntoView>,
) {
    let Anchor = move || {
        mview! {
            " "
            a href={move || format!("#{}", text) } (
                {text}
            )
            " "
        }
    };
    let Section = move |children: Children| {
        mview! {
            section id={text} class="border-src-350 target:bg-src-800 m-4 rounded-md border p-4" (
                {children()}
            )
        }
    };

    return (Anchor, Section);
}
