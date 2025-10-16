use leptos::prelude::*;

#[component]
pub fn DurationEditor(set_duration_mins: WriteSignal<i32>) -> impl IntoView {
    view! {
        <div>
            <button on:click=move |_| {
                set_duration_mins.update(|val| *val = (*val - 60).max(0))
            }>"-1 h"</button>
            <button on:click=move |_| {
                set_duration_mins.update(|val| *val = (*val - 10).max(0))
            }>"-10 m"</button>
            <button on:click=move |_| set_duration_mins.update(|val| *val += 10)>"+10 m"</button>
            <button on:click=move |_| set_duration_mins.update(|val| *val += 60)>"+1 h"</button>
        </div>
    }
}
