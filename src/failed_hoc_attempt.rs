#![feature(impl_trait_in_bindings)]
#![feature(impl_trait_in_assoc_type)]
#![feature(impl_trait_in_fn_trait_return)]
#![feature(inherent_associated_types)]

use leptos::prelude::*;
use leptos_mview::mview;

#[component]
pub fn Anchor(is: &'static str) -> impl IntoView {
    mview! {
        a href=f["#{is}"] ( {is} )
    }
}

#[component]
pub fn Section(is: &'static str, children: Children) -> impl IntoView {
    mview! {
        section id={is}  (
            {children()}
        )
    }
}

#[component]
pub fn Aside(is: &'static str, children: Children) -> impl IntoView {
    mview! {
        aside id={is} class="border-src-350 target:bg-src-800 m-4 rounded-md border p-4" (
            {children()}
        )
    }
}

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

#[component]
fn Anchor(text: &'static str) -> impl IntoView {
    mview! {
        a href={format!("#{}", &text)} ( {format!("{}", text)} )
    }
}

#[component]
fn Section(text: &'static str, children: Children) -> impl IntoView {
    mview! {
        section id={text} class="border-src-350 target:bg-src-800 m-4 rounded-md border p-4" (
            {children()}
        )
    }
}

#[component]
fn Aside(text: &'static str, children: Children) -> impl IntoView {
    mview! {
        aside id={text} class="border-src-350 target:bg-src-800 m-4 rounded-md border p-4" (
            {children()}
        )
    }
}

pub struct Anchored<C, CC> {
    // pub text: &'static str,
    pub Anchor: C,
    pub Section: CC,
    pub Aside: CC,
}

impl Anchored<C, CC> {
    type C = impl Fn() -> impl IntoView;

    pub fn new(text: &'static str) -> Self {
        #[component]
        fn Anchor() -> impl IntoView {
            mview! {
                a href={format!("#{}", &text)} ( {text} )
            }
        }
        #[component]
        fn Section(children: Children) -> impl IntoView {
            mview! {
                section id={self.0} class="border-src-350 target:bg-src-800 m-4 rounded-md border p-4" (
                    {children()}
                )
            }
        }
        #[component]
        fn Aside(children: Children) -> impl IntoView {
            mview! {
                aside id={self.0} class="border-src-350 target:bg-src-800 m-4 rounded-md border p-4" (
                    {children()}
                )
            }
        }

        Self {
            Anchor,
            Section,
            Aside,
        }
    }

    pub fn Anchor(&self) -> impl IntoView {
        mview! {
            a href={format!("#{}", &self.0)} ( {self.0} )
        }
    }
    #[component]
    pub fn Section(&self, children: Children) -> impl IntoView {
        mview! {
            section id={self.0} class="border-src-350 target:bg-src-800 m-4 rounded-md border p-4" (
                {children()}
            )
        }
    }
    #[component]
    pub fn Aside(&self, children: Children) -> impl IntoView {
        mview! {
            aside id={self.0} class="border-src-350 target:bg-src-800 m-4 rounded-md border p-4" (
                {children()}
            )
        }
    }
}
