use crate::components::{GameBoard, GameProvider};
use crate::game::model::Game;
use yew::prelude::*;

#[component]
pub fn App() -> Html {
    let game = use_state(|| None);

    let on_reset_click = {
        let game = game.clone();
        Callback::from(move |_: MouseEvent| game.set(None))
    };

    let game_specified = {
        let game = game.clone();
        Callback::from(move |specified_game: Game| game.set(Some(specified_game)))
    };

    html! {
        <main class="flex flex-col items-center gap-4 py-6 px-4">
            <h1 class="text-3xl font-bold text-center">{"Numbers Round Solver"}</h1>
            { match &*game {
                None => html! {
                    <GameProvider on_game_specified={ game_specified } />
                },
                Some(game) => html! {
                    <GameBoard game={ game.clone() } on_reset={ on_reset_click } />
                },
            }}
        </main>
    }
}
