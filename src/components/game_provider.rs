use crate::components::ManualEntry;
use crate::game::board::Board;
use crate::game::model::Game;
use rand::Rng;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GameProviderProps {
    pub on_game_specified: Callback<Game>,
}

#[component]
pub fn GameProvider(props: &GameProviderProps) -> Html {
    let show_custom_split = use_state(|| false);
    let show_manual_entry = use_state(|| false);

    let generate_random_game = {
        let on_game_specified = props.on_game_specified.clone();

        Callback::from(move |_: MouseEvent| {
            let game = Game::default();
            on_game_specified.emit(game);
        })
    };

    let open_custom_split = {
        let show_custom_split = show_custom_split.clone();
        Callback::from(move |_: MouseEvent| {
            show_custom_split.set(true);
        })
    };

    let close_custom_split = {
        let show_custom_split = show_custom_split.clone();
        Callback::from(move |_: MouseEvent| {
            show_custom_split.set(false);
        })
    };

    let open_manual_entry = {
        let show_manual_entry = show_manual_entry.clone();
        Callback::from(move |_: MouseEvent| {
            show_manual_entry.set(true);
        })
    };

    let close_manual_entry = {
        let show_manual_entry = show_manual_entry.clone();
        Callback::from(move |_: MouseEvent| {
            show_manual_entry.set(false);
        })
    };

    html! {
        <div class="flex flex-col items-center gap-6 p-4">
            <h2 class="text-2xl font-bold text-center">{"Choose Numbers Round Setup"}</h2>

            if *show_custom_split {
                <div aria-label="Custom split setup" class="flex flex-col items-center gap-6 w-full max-w-2xl">
                    <h2 class="text-2xl font-bold text-center">{"Choose Your Number Split"}</h2>
                    <div class="flex flex-col items-center gap-2">
                        <p class="text-sm text-gray-500">{"How many large numbers?"}</p>
                        <div role="group" aria-label="Number of large numbers" class="inline-flex rounded-lg border-2 border-blue-500 overflow-hidden divide-x-2 divide-blue-500">
                            { for (0u8..=4).map(|large_count| {
                                let small_count = 6 - large_count;
                                let on_game_specified = props.on_game_specified.clone();
                                html! {
                                    <button
                                        class="px-6 py-3 font-bold text-xl text-blue-700 bg-white hover:bg-blue-100 transition-colors duration-150 cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-blue-600"
                                        aria-label={format!("Select {} large number(s)", large_count)}
                                        onclick={Callback::from(move |_: MouseEvent| {
                                            let board = Board::random_with_number_mix_specified(small_count, large_count).unwrap();
                                            let mut rng = rand::rng();
                                            let target = rng.random_range(1u16..=999u16);
                                            let game = Game::new(board, target).unwrap();
                                            on_game_specified.emit(game);
                                        })}
                                    >
                                        {large_count}
                                    </button>
                                }
                            }) }
                        </div>
                    </div>
                    <button
                        class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-3 px-6 rounded-lg shadow-md transition-colors duration-200 cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-gray-600"
                        aria-label="Back to game options"
                        onclick={close_custom_split}
                    >
                        {"Back"}
                    </button>
                </div>
            } else if *show_manual_entry {
                <ManualEntry
                    on_game_specified={props.on_game_specified.clone()}
                    on_back={close_manual_entry}
                />
            } else {
                <div class="flex flex-wrap gap-4 justify-center w-full max-w-2xl">
                    <button
                        class="flex-1 min-w-[200px] bg-blue-500 hover:bg-blue-700 text-white font-bold py-6 px-8 rounded-lg shadow-md transition-colors duration-200 cursor-pointer flex flex-col items-center gap-3 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-blue-600"
                        onclick={generate_random_game}
                        aria-label="Generate random game"
                    >
                        <svg aria-hidden="true" class="w-12 h-12" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path>
                        </svg><span>{"Random Game"}</span>
                    </button>

                    <button
                        class="flex-1 min-w-[200px] bg-blue-500 hover:bg-blue-700 text-white font-bold py-6 px-8 rounded-lg shadow-md transition-colors duration-200 cursor-pointer flex flex-col items-center gap-3 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-blue-600"
                        aria-label="Simulate a round by choosing your number mix"
                        onclick={open_custom_split}
                    >
                        <svg aria-hidden="true" class="w-12 h-12" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4"></path>
                        </svg>
                        <span>{"Simulate Round"}</span>
                        <span class="text-sm font-normal">{"Choose large vs small number mix"}</span>
                    </button>

                    <button
                        class="flex-1 min-w-[200px] bg-purple-500 hover:bg-purple-700 text-white font-bold py-6 px-8 rounded-lg shadow-md transition-colors duration-200 cursor-pointer flex flex-col items-center gap-3 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-purple-600"
                        aria-label="Specify complete game setup"
                        onclick={open_manual_entry}
                    >
                        <svg aria-hidden="true" class="w-12 h-12" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
                        </svg>
                        <span>{"Manual Entry"}</span>
                    </button>
                </div>
            }
        </div>
    }
}
