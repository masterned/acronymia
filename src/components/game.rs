use ::leptos::*;

use crate::sse::*;
use crate::types::*;
use crate::components::text_input::*;
use crate::components::reset_button::*;
use crate::components::utils::*;
use crate::typed_context::*;

mod judging;
mod setup;
mod results;
mod submission;
mod context;
use self::judging::*;
use self::setup::*;
use self::results::*;
use self::submission::*;
use self::context::*;


#[component]
pub fn Game(cx: Scope) -> impl IntoView {
    provide_game_context(cx);
    let player_id = use_typed_context::<Signal_PlayerId>(cx);
    let game_step = create_sse_signal::<GameStep>(cx);
    let game_step = create_memo(cx, move |_| game_step());
    view! {
        cx,
        <Debug>
            <div>
                <h1 class="font-bold font-xl">"Begin Debug"</h1>
                <ResetButton/>

                <p>"Override player id: "</p>
                <TextInput
                    default=player_id().unwrap_or("".to_string())
                    on_input=move |text| player_id.set(Some(text))
                />
                <h1 class="font-bold font-xl">"End Debug"</h1>
            </div>
        </Debug>
        { move || match game_step() {
            None => view! {cx, <><GameNotFound /></>},
            Some(GameStep::Setup) => view! { cx, <><GameSetup /></> },
            Some(GameStep::Submission) => view! { cx, <><GameSubmission /></> },
            Some(GameStep::Judging) => view! { cx, <><GameJudging /></> },
            Some(GameStep::Results) => view! { cx, <><GameResults /></> },
        }}
    }
}

#[component]
fn GameNotFound(_cx: Scope) -> impl IntoView {
    view! {
        cx,
        "Game not found!"
    }
}
