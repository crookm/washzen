use leptos::prelude::*;
use leptos_meta::*;

mod components;
mod formatting;

use crate::components::duration_editor::DurationEditor;
use crate::formatting::{format_datetime, format_duration};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        <Title text="WashZen" />

        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Link rel="canonical" href="https://mattcrook.nz/projects/washwhen.html" />

        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>
            <Calculator />
        </ErrorBoundary>
    }
}

#[component]
fn Calculator() -> impl IntoView {
    let (target_end_from_now_mins, set_target_end_from_now_mins) = signal(120);
    let (cycle_length_mins, set_cycle_length_mins) = signal(90);
    let delay_to_set_mins = move || target_end_from_now_mins.get() - cycle_length_mins.get();

    let target_time = move || format_datetime(target_end_from_now_mins.get());
    let target_duration = move || format_duration(target_end_from_now_mins.get());
    let cycle = move || format_duration(cycle_length_mins.get());
    let delay = move || format_duration(delay_to_set_mins());

    let result_class = move || {
        if delay_to_set_mins() >= 0 {
            "results success"
        } else {
            "results error"
        }
    };

    view! {
        <section>
            <label>
                "Target end time: " <strong>{move || target_time()}</strong> " (in "
                {move || target_duration()} ")" <p>"When you want the cycle to finish"</p>
            </label>
            <DurationEditor set_duration_mins=set_target_end_from_now_mins />
        </section>

        <section>
            <label>
                "Cycle length: " <strong>{move || cycle()}</strong>
                <p>"How long the appliance takes for the chosen cycle"</p>
            </label>
            <DurationEditor set_duration_mins=set_cycle_length_mins />
        </section>

        <section class=move || result_class()>
            <h2>"Delay to set"</h2>
            <p>
                {move || {
                    if delay_to_set_mins() >= 0 {
                        view! { <span>{delay()}</span> }.into_any()
                    } else {
                        let overage = format_duration((-delay_to_set_mins()).max(0));
                        view! {
                            <span>
                                "Target end time is earlier than the cycle by "
                                <strong>{overage}</strong>"."
                            </span>
                        }
                            .into_any()
                    }
                }}
            </p>
        </section>
    }
}
