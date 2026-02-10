use leptos::prelude::*;
use leptos_mview::mview;
use tracing::info;

#[component]
fn ProgressBar(
    #[prop(default = 100)] max: u16,
    progress: impl Fn() -> i32 + Send + Sync + 'static,
) -> impl IntoView {
    mview! {
        progress {max} value={progress};
    }
}

#[component]
pub fn Counters() -> impl IntoView {
    let (count, set_count) = signal(0);
    let increment = move |_| {
        info!("incremented");
        *set_count.write() += 1;
    };

    let double_count = move || count() * 2;
    let progress = move || count() * 3;

    mview! {
        // a href=[| count()] ("foo")

        // button on:click=[_| *set_count.write() += 1] (
        //     "Clickest moi: " {count}
        // )
        button on:click={move |_| *set_count.write() += 1} (
            "old Clickest moi: " {count}
        )


        // button on:click={increment} class:"bg-red-d"=[|count() % 2 == 1] (
        //     "Click me: " {count}
        // )
        button on:click={increment} class:"bg-red-d"=[count() % 2 == 1] (
            "old Click me: " {count}
        )


        // button
        //     on:click=[_| set_count(count() + 1)]
        //     class:"bg-green-d"=[|count() % 2 == 0]
        // (
        //     "Or click me: " {count}
        // )
        button
            on:click={move |_| set_count(count() + 1)}
            class:"bg-green-d"=[count() % 2 == 0]
        (
            "old Or click me: " {count}
        )


        // button on:click={increment} class:bg-blue-d=[|count() % 2 == 1] (
        //     "Or even click me: " {count}
        // )
        button on:click={increment} class:bg-blue-d=[count() % 2 == 1] (
            "old Or even click me: " {count}
        )


        p ("Double count:" {double_count} )
        ProgressBar progress={count};
        ProgressBar max=50 progress={double_count};
        ProgressBar max=25 {progress};
        a href="foo" ("foo")
        a href="bar" ("bar")
    }
}
