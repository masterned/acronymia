#![feature(type_alias_impl_trait)]
#![feature(unboxed_closures)]
#![feature(let_chains)]
#![feature(stmt_expr_attributes)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::let_underscore_untyped)]
#![allow(clippy::default_trait_access)]
#![allow(clippy::uninlined_format_args)]
pub mod api;
pub mod components;
pub mod sse;
pub mod typed_context;
pub mod types;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use cfg_if::cfg_if;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    use crate::components::game::*;
    use crate::components::timer::*;

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);
    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/acronymia.css"/>

        // sets the document title
        <Title text="Acronymia"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route
                        path=""
                        view=move |cx| view! { cx, <Game/> }
                    />
                    <Route
                        path="timer"
                        view=move |cx| view! { cx, <Timer initial=2 /> }
                    />
                </Routes>
            </main>
        </Router>
    }
}

cfg_if! {
  if #[cfg(feature = "hydrate")] {

    use wasm_bindgen::prelude::wasm_bindgen;

      #[wasm_bindgen]
      pub fn hydrate() {
        use leptos::*;

        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(move |cx| {
            view! { cx, <App/> }
        });
      }
  }
}
