use leptos::*;
use std::time::Duration;

#[component]
pub fn Timer(cx: Scope) -> impl IntoView {
    let seconds = timer(cx, 60);
    view! { cx, "Seconds: "{seconds}}
}

fn timer(cx: Scope, initial: u32) -> RwSignal<u32> {
    let seconds = create_rw_signal(cx, initial);
    create_effect(cx, move |_| {
        let handle = set_interval(
            move || {
                let s = seconds.get();
                if s > 0 {
                    seconds.set(s - 1);
                }
            },
            Duration::new(1, 0),
        );
        log::debug!("{:?}", &handle);
        on_cleanup(cx, move || {
            _ = handle.map(|h| h.clear());
        });
    });

    seconds
}


