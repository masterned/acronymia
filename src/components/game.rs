use ::leptos::*;

use crate::components::reset_button::*;
use crate::components::utils::*;
use crate::typed_context::*;
use crate::types::*;

mod acronym;
mod context;
mod judging;
mod player_roster;
mod results;
mod setup;
mod submission;
mod timer;
pub mod utils;
use self::context::*;
use self::judging::*;
use self::player_roster::*;
use self::results::*;
use self::setup::*;
use self::submission::*;
use self::utils::state::*;

#[component]
pub fn Game(cx: Scope) -> impl IntoView {
    provide_game_context(cx);
    let game_step = create_memo(cx, move |_| game_state(cx).with(|g| g.step.clone()));
    let debug_region_expanded = create_rw_signal(cx, false);

    view! {
        cx,
        <div class="flex flex-col items-start mx-20 my-4 gap-4">
           <h1 class="text-xl font-bold">"Acronymia"</h1>
            { move || match game_step() {
                GameStep::Setup => view! { cx, <><GameSetup /></> },
                GameStep::Submission => view! { cx, <><GameSubmission /></> },
                GameStep::Judging => view! { cx, <><GameJudging /></> },
                GameStep::Results => view! { cx, <><GameResults /></> },
            }}
            {debug_view(cx, view! {cx,
                <button
                    class="border rounded p-2 bg-slate-200"
                    on:click=move |_| debug_region_expanded.update(|b| *b = !*b)
                >
                    "Toggle Debug View"
                </button>
                <When predicate=debug_region_expanded >
                    <div class="flex flex-col items-start gap-4">
                        <h1 class="font-bold font-xl">"Begin Debug"</h1>
                        <p>
                            "You are "<PlayerName />
                        </p>
                        <PlayerRoster />
                        <div>{move || format!("WS game_state = {:#?}", game_state(cx).get())}</div>
                        <ResetButton />
                        <h1 class="font-bold font-xl">"End Debug"</h1>
                    </div>
                </When>
            })}
        </div>
    }
}

#[component]
fn PlayerName(cx: Scope) -> impl IntoView {
    //<span class="inline font-bold">
    move || {
        get_name(cx).map_or(
            view! { cx,
                <span class="inline font-bold text-red-300">
                    "nobody"
                </span>
            },
            |name| {
                view! { cx,
                    <span class="inline font-bold text-green-300">
                        {name}
                    </span>
                }
            },
        )
    }
}

fn get_name(cx: Scope) -> Option<String> {
    use_typed_context::<Signal_PlayerId>(cx).with(|id| {
        id.as_ref().and_then(|id| {
            game_state(cx).with(|g| {
                g.players
                    .iter()
                    .find(|p| p.id == *id)
                    .map(|p| p.name.clone())
            })
        })
    })
}
