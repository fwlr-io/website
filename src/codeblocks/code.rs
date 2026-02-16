#[component]
pub fn Code(code: &'static str) -> impl IntoView {
    view! {
        <div class="flex justify-center bg-black border py-1g border-y-grey">
            <div
                class="font-mono whitespace-pre w-xl text-dim-white"
                inner_html=code
            />
        </div>
    }
}
