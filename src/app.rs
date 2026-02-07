use leptos::prelude::*;
use leptos_mview::mview;
use tracing::info;

#[component]
fn ProgressBar(
    #[prop(default = 100)] max: u16,
    // #[prop(into)] progress: Signal<i32>,
    progress: impl Fn() -> i32 + Send + Sync + 'static,
) -> impl IntoView {
    mview! {
        progress {max} value={progress};
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let increment = move |_| {
        info!("incremented");
        *set_count.write() += 1;
    };
    let is_odd = move || count() % 2 == 1;
    let double_count = move || count() * 2;

    mview! {
        button on:click={increment} class:bg-red-500=[count() % 2 == 1] (
            "Click me: " {count}
        )
        button
            on:click={move |_| set_count(count() + 1)}
                class:"bg-green-500"=[count() % 2 == 0]
        (
            "Or click me: " {count}
        )
        button on:click={increment} class:bg-blue-500={is_odd} (
            "Or even click me: " {count}
        )
        p ("Double count:" {double_count} )
        // progress max="50" value={count};
        // progress max="100" value={double_count};
        ProgressBar progress={count};
        ProgressBar max=50 progress={double_count};
    }
    // view! {
    //     <button
    //          on:click={increment}
    //          class=("bg-red-500", move || count() % 2 == 1)>
    //
    //         class=(["bg-red-500", "text-red-500"], move || count() % 2 == 1)
    //         "Click me: "
    //         {count}
    //     </button>
    //     <p>"Double count: " {move || count() * 2}</p>
    // }
}
