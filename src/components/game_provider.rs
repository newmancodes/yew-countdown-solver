use crate::game::game::Game;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GameProviderProps {
    pub on_game_specified: Callback<Game>,
}

#[component]
pub fn GameProvider(props: &GameProviderProps) -> Html {
    let generate_random_game = {
        let on_game_specified = props.on_game_specified.clone();

        Callback::from(move |_: MouseEvent| {
            let game = Game::default();
            on_game_specified.emit(game);
        })
    };

    html! {
        <div class="flex flex-col items-center gap-6 p-4">
            <h1 class="text-3xl font-bold text-center">{"Choose Numbers Round Setup"}</h1>

            <div class="flex flex-wrap gap-4 justify-center w-full max-w-2xl">
                <button
                    class="flex-1 min-w-[200px] bg-green-500 hover:bg-green-700 text-white font-bold py-6 px-8 rounded-lg shadow-md transition-colors duration-200 cursor-pointer flex flex-col items-center gap-3"
                    onclick={generate_random_game}
                    aria-label="Generate random game"
                >
                    <svg class="w-12 h-12" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path>
                    </svg><span>{"Random Game"}</span>
                </button>

                <button
                    class="flex-1 min-w-[200px] bg-blue-500 hover:bg-blue-700 text-white font-bold py-6 px-8 rounded-lg shadow-md transition-colors duration-200 cursor-pointer flex flex-col items-center gap-3"
                    aria-label="Create game with number constraints"
                >
                    <svg class="w-12 h-12" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4"></path>
                    </svg>
                    <span>{"Custom Split"}</span>
                </button>

                <button
                    class="flex-1 min-w-[200px] bg-purple-500 hover:bg-purple-700 text-white font-bold py-6 px-8 rounded-lg shadow-md transition-colors duration-200 cursor-pointer flex flex-col items-center gap-3"
                    aria-label="Specify complete game setup"
                >
                    <svg class="w-12 h-12" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
                    </svg>
                    <span>{"Manual Entry"}</span>
                </button>
            </div>
        </div>
    }
}
