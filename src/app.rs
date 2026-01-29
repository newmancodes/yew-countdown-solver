use crate::game::game::Game;
use crate::solver::iterative_deepening::IterativeDeepeningSolver;
use crate::solver::solver::Solver;
use gloo_timers::future::sleep;
use std::time::Duration;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[component]
pub fn App() -> Html {
    let game = use_state(|| Some(Game::default()));

    let on_reset_click = {
        let game = game.clone();
        Callback::from(move |_: MouseEvent| game.set(None))
    };

    html! {
        <main>
            <GameBoard game={ (*game).clone() } on_reset={ on_reset_click } />
        </main>
    }
}

#[derive(Properties, PartialEq)]
pub struct GameBoardProps {
    pub game: Option<Game>,
    pub on_reset: Callback<MouseEvent>,
}

#[component]
pub fn GameBoard(props: &GameBoardProps) -> Html {
    let solving = use_state(|| false);
    let solved = use_state(|| false);

    let on_solve_click = {
        if let Some(game) = &props.game {
            let solving = solving.clone();
            let solved = solved.clone();
            let game = game.clone();

            Callback::from(move |_| {
                let solving = solving.clone();
                let solved = solved.clone();
                let game = game.clone();

                solving.set(true);

                spawn_local(async move {
                    sleep(Duration::from_secs(2)).await;

                    let solver = IterativeDeepeningSolver::new(&game);
                    let solution = solver.solve();
                    if solution.is_some() {
                        solved.set(true);
                    }
                    solving.set(false);
                });
            })
        } else {
            Callback::from(move |_| {})
        }
    };

    html! {
        if let Some(game) = &props.game {
            <div class="flex flex-col items-center gap-6 p-4">
                <section aria-label="Game board" class="w-full max-w-md">
                    <div class="bg-blue-600 text-white text-2xl font-bold text-center py-4 px-6 border-2 border-gray-600 rounded-lg mb-4">
                        <span aria-label="Target number">{ game.target() }</span>
                    </div>

                    <div class="grid grid-cols-6 sm:grid-cols-3 md:grid-cols-2 lg:grid-cols-6 gap-3" role="list" aria-label="Available numbers">
                        { for game.board().numbers().iter().map(|number| {
                            html! {
                                <div role="listitem" class="bg-white text-black text-xl font-semibold flex items-center justify-center py-4 px-6 border-2 border-gray-400 rounded-lg">
                                    { *number }
                                </div>
                            }
                        })}
                    </div>
                </section>

                <div class="flex flex-wrap gap-3 justify-center">
                    <button
                        class="bg-blue-500 hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed text-white font-bold py-3 px-8 rounded-lg shadow-md transition-colors duration-200 cursor-pointer"
                        onclick={on_solve_click}
                        disabled={*solving}
                        aria-busy={(*solving).to_string()}
                        aria-label="Solve game"
                    >
                        { if *solving { "Solving..." } else { "Solve" } }
                    </button>

                    <button
                        class="bg-gray-500 hover:bg-gray-700 disabled:bg-gray-400 disabled:cursor-not-allowed text-white font-bold py-3 px-8 rounded-lg shadow-md transition-colors duration-200 cursor-pointer flex items-center gap-2"
                        onclick={props.on_reset.clone()}
                        disabled={*solving}
                        aria-busy={(*solving).to_string()}
                        aria-label="Reset game"
                    >
                        <span>{"↻"}</span>
                        <span>{"Reset"}</span>
                    </button>
                </div>
            </div>
        }
    }
}
