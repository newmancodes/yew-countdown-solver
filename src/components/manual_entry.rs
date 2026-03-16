use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_countdown_solver::game::board::BoardBuilder;
use yew_countdown_solver::game::model::Game;

#[derive(Clone, Default)]
struct ManualEntryState {
    selected: Vec<u32>,
    target_str: String,
}

#[derive(Properties, PartialEq)]
pub struct ManualEntryProps {
    pub on_game_specified: Callback<Game>,
    pub on_back: Callback<MouseEvent>,
}

#[component]
pub fn ManualEntry(props: &ManualEntryProps) -> Html {
    let state = use_state(ManualEntryState::default);

    let board_result = state
        .selected
        .iter()
        .try_fold(BoardBuilder::new(), |builder, &n| builder.add_number(n))
        .and_then(|builder| builder.build());

    let target: Option<u32> = state
        .target_str
        .parse::<u32>()
        .ok()
        .filter(|&t| (1..=999).contains(&t));

    let target_error = if state.target_str.is_empty() {
        None
    } else if target.is_none() {
        Some("Target must be between 1 and 999")
    } else {
        None
    };

    let can_confirm = board_result.is_ok() && target.is_some();

    let on_target_input = {
        let state = state.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_state = (*state).clone();
            new_state.target_str = input.value();
            state.set(new_state);
        })
    };

    let on_confirm = {
        let state = state.clone();
        let on_game_specified = props.on_game_specified.clone();
        Callback::from(move |_: MouseEvent| {
            let board = state
                .selected
                .iter()
                .try_fold(BoardBuilder::new(), |builder, &n| builder.add_number(n))
                .and_then(|builder| builder.build());
            let target: Option<u32> = state
                .target_str
                .parse::<u32>()
                .ok()
                .filter(|&t| (1..=999).contains(&t));
            if let (Ok(board), Some(target)) = (board, target) {
                if let Ok(game) = Game::new(board, target) {
                    on_game_specified.emit(game);
                }
            }
        })
    };

    html! {
        <div aria-label="Manual entry setup" class="flex flex-col items-center gap-6 p-4 w-full max-w-2xl">
            <h2 class="text-2xl font-bold text-center">{"Manual Entry"}</h2>

            <div class="flex flex-col items-center gap-2">
                <label for="target-input" class="font-semibold">{"Target (1–999)"}</label>
                <input
                    type="number"
                    id="target-input"
                    min="1"
                    max="999"
                    class="border-2 border-gray-400 rounded-lg px-4 py-2 text-xl text-center w-32 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-blue-600"
                    value={state.target_str.clone()}
                    oninput={on_target_input}
                />
                if let Some(err) = target_error {
                    <p class="text-sm text-red-600">{err}</p>
                }
            </div>

            <div class="flex flex-col items-center gap-4 w-full">
                <div class="flex flex-col items-center gap-2 w-full">
                    <p class="text-sm font-semibold text-gray-600">{"Large numbers — select up to 1 of each"}</p>
                    <div class="flex flex-wrap gap-3 justify-center">
                        { for [25u32, 50, 75, 100].into_iter().map(|n| {
                            let count = state.selected.iter().filter(|&&x| x == n).count();
                            let can_add = state.selected.len() < 6 && count < 1;
                            let state_clone = state.clone();
                            html! {
                                <button
                                    class="bg-blue-500 hover:bg-blue-700 disabled:bg-gray-300 disabled:cursor-not-allowed text-white font-bold py-3 px-4 rounded-lg shadow-md transition-colors duration-200 cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-blue-600"
                                    aria-label={format!("number {}", n)}
                                    disabled={!can_add}
                                    onclick={Callback::from(move |_: MouseEvent| {
                                        let mut new_state = (*state_clone).clone();
                                        new_state.selected.push(n);
                                        state_clone.set(new_state);
                                    })}
                                >
                                    {n}
                                </button>
                            }
                        }) }
                    </div>
                </div>
                <div class="flex flex-col items-center gap-2 w-full">
                    <p class="text-sm font-semibold text-gray-600">{"Small numbers — select up to 2 of each"}</p>
                    <div class="flex flex-wrap gap-2 justify-center">
                        { for (1u32..=10).map(|n| {
                            let count = state.selected.iter().filter(|&&x| x == n).count();
                            let can_add = state.selected.len() < 6 && count < 2;
                            let state_clone = state.clone();
                            html! {
                                <button
                                    class="bg-blue-500 hover:bg-blue-700 disabled:bg-gray-300 disabled:cursor-not-allowed text-white font-bold py-3 px-4 rounded-lg shadow-md transition-colors duration-200 cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-blue-600"
                                    aria-label={format!("number {}", n)}
                                    disabled={!can_add}
                                    onclick={Callback::from(move |_: MouseEvent| {
                                        let mut new_state = (*state_clone).clone();
                                        new_state.selected.push(n);
                                        state_clone.set(new_state);
                                    })}
                                >
                                    {n}
                                </button>
                            }
                        }) }
                    </div>
                </div>
            </div>

            <div class="grid grid-cols-6 gap-3 w-full max-w-md">
                { for (0..6usize).map(|i| {
                    if i < state.selected.len() {
                        let n = state.selected[i];
                        let state_clone = state.clone();
                        html! {
                            <button
                                class="bg-white text-black text-xl font-semibold flex items-center justify-center py-4 px-2 border-2 border-gray-400 rounded-lg hover:bg-red-100 cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-blue-600"
                                aria-label={format!("remove {}", n)}
                                onclick={Callback::from(move |_: MouseEvent| {
                                    let mut new_state = (*state_clone).clone();
                                    new_state.selected.remove(i);
                                    state_clone.set(new_state);
                                })}
                            >
                                {n}
                            </button>
                        }
                    } else {
                        html! {
                            <div class="bg-gray-100 border-2 border-dashed border-gray-300 rounded-lg py-4 flex items-center justify-center">
                            </div>
                        }
                    }
                }) }
            </div>

            if state.selected.len() < 6 {
                <p class="text-gray-600">{format!("Select {} more number(s)", 6 - state.selected.len())}</p>
            }

            <div class="flex gap-4">
                <button
                    class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-3 px-6 rounded-lg shadow-md transition-colors duration-200 cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-gray-600"
                    aria-label="Back to game options"
                    onclick={props.on_back.clone()}
                >
                    {"Back"}
                </button>
                <button
                    class="bg-purple-500 hover:bg-purple-700 disabled:bg-gray-400 disabled:cursor-not-allowed text-white font-bold py-3 px-6 rounded-lg shadow-md transition-colors duration-200 cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-purple-600"
                    aria-label="Confirm game"
                    disabled={!can_confirm}
                    onclick={on_confirm}
                >
                    {"Confirm"}
                </button>
            </div>
        </div>
    }
}
