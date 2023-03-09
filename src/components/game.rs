use crate::components::text_input::*;
use crate::components::utils::*;
use crate::sse::*;
use crate::typed_context::*;
use crate::types::*;
use crate::*;
use leptos::*;
use uuid::*;

define_context!(Signal_PlayerId, RwSignal<Option<String>>);
define_context!(Signal_PlayerName, RwSignal<String>);
define_context!(Action_JoinGame, Action<(), Result<(), ServerFnError>>);

/// a signal for the player id
/// that caches its value inside local storage
fn signal_player_id(cx: Scope) -> RwSignal<Option<String>> {
    const STORAGE_KEY: &str = "acronymia-player-id";

    let player_id: RwSignal<Option<String>> = create_rw_signal(cx, None);

    // this only runs once because it does not depend on any reactive values
    // but its wrapped in create_effect to ensure it runs on the client side
    create_effect(cx, move |_| {
        if player_id.get().is_some() {
            return ();
        }
        let new_player_id = move |storage: web_sys::Storage| {
            let id = Uuid::new_v4().to_string();
            _ = storage.set_item(STORAGE_KEY, &id);
            player_id.set(Some(id));
        };
        match window().local_storage() {
            Ok(Some(storage)) => match storage.get_item(STORAGE_KEY) {
                Ok(Some(id)) => player_id.set(Some(id)),
                _ => new_player_id(storage),
            },
            _ => (),
        }
    });

    player_id
}

fn provide_game_context(cx: Scope) {
    provide_sse_stream(cx);
    let player_id = signal_player_id(cx);
    provide_typed_context::<Signal_PlayerId>(cx, player_id);

    let player_name = create_rw_signal(cx, "".to_string());
    provide_typed_context::<Signal_PlayerName>(cx, player_name);

    let join_game = create_action(cx, move |_: &()| {
        api::join_game(player_id().unwrap_or("".to_owned()), player_name())
    });
    provide_typed_context::<Action_JoinGame>(cx, join_game);
}

#[component]
pub fn Game(cx: Scope) -> impl IntoView {
    provide_game_context(cx);
    let game_step = create_sse_signal::<GameStep>(cx);
    let game_step = create_memo(cx, move |_| game_step());
    view! {
        cx,

        <Transition
            fallback=move || view! { cx, "Loading" }
        >
            { move || match game_step() {
                None => view! {cx, <><GameNotFound /></>},
                Some(GameStep::Setup) => view! { cx, <><GameSetup /></> },
                Some(GameStep::Submission) => view! { cx, <><GameSubmission /></> },
                Some(GameStep::Judging) => view! { cx, <><GameJudging /></> },
                Some(GameStep::Results) => view! { cx, <><GameResults /></> },
            }}
        </Transition>
    }
}

#[component]
fn GameNotFound(_cx: Scope) -> impl IntoView {
    view! {
        cx,
        "Game not found!"
    }
}

#[component]
fn GameSetup(cx: Scope) -> impl IntoView {
    let player_id = use_typed_context::<Signal_PlayerId>(cx);
    let player_name = use_typed_context::<Signal_PlayerName>(cx);
    let players = create_sse_signal::<Vec<Player>>(cx);
    let join_game = use_typed_context::<Action_JoinGame>(cx);

    view! {
        cx,
        <Debug>
            <div>
                <h1 class="font-bold font-xl">"Begin Debug"</h1>
                "Override player id (Debug only): "
                <TextInput
                    default=player_id.get().unwrap_or("".to_string())
                    on_input=move |text| player_id.set(Some(text))
                />
                <h1 class="font-bold font-xl">"End Debug"</h1>
            </div>
        </Debug>
        <div>
            "Pick a Nickname to join: "
            <TextInput
                default=player_name()
                on_input=move |text| player_name.set(text)
            />
            <button
                class="border rounded p-2 m-2 bg-blue-300 border-slate-200"
                prop:disabled=MaybeSignal::derive(cx, move|| player_id().is_none())
                on:click=move |_| join_game.dispatch(())
            >
                "Join!"
            </button>
            <p> "Players: "</p>
            <ul class="list-inside list-disc" >
                {move|| players()
                    .into_iter()
                    .flatten()
                    .map(|p| view! {cx, <li>{p.name}</li>})
                    .collect::<Vec<_>>()
                }
            </ul>
        </div>
    }
}

#[component]
fn GameSubmission(_cx: Scope) -> impl IntoView {
    view! {
        cx,
        "Submission!"
    }
}

#[component]
fn GameJudging(_cx: Scope) -> impl IntoView {
    view! {
        cx,
        "Judging!"
    }
}

#[component]
fn GameResults(_cx: Scope) -> impl IntoView {
    view! {
        cx,
        "Results!"
    }
}
