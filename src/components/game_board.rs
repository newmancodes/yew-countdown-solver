use crate::game::game::{Board, Game};
use crate::solver::iterative_deepening::IterativeDeepeningSolver;
use crate::solver::solver::{Operator, Solver};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GameBoardProps {
    pub game: Game,
    pub on_reset: Callback<MouseEvent>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SolutionState {
    NotAttempted,
    Solving,
    Solved(crate::solver::solver::Solution<Game, Board>),
    NotFound,
}

#[component]
pub fn GameBoard(props: &GameBoardProps) -> Html {
    let solution_state = use_state(|| SolutionState::NotAttempted);
    let game = props.game.clone();

    let on_solve_click = {
        let solution_state = solution_state.clone();
        let game = game.clone();

        Callback::from(move |_| {
            let solution_state = solution_state.clone();
            let game = game.clone();

            solution_state.set(SolutionState::Solving);

            let solver = IterativeDeepeningSolver::new(&game);
            if let Some(solution) = solver.solve() {
                tracing::info!(
                    "Found solution for game {:?} in {} operations",
                    game,
                    solution.number_of_operations(),
                );
                solution_state.set(SolutionState::Solved(solution));
            } else {
                tracing::info!("No solution found for game {:?}", game);
                solution_state.set(SolutionState::NotFound);
            }
        })
    };

    html! {
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
                    disabled={*solution_state != SolutionState::NotAttempted}
                    aria-busy={(*solution_state != SolutionState::NotAttempted).to_string()}
                    aria-label="Solve game"
                >
                    { if *solution_state == SolutionState::Solving { "Solving..." } else { "Solve" } }
                </button>

                <button
                    class="bg-gray-500 hover:bg-gray-700 disabled:bg-gray-400 disabled:cursor-not-allowed text-white font-bold py-3 px-8 rounded-lg shadow-md transition-colors duration-200 cursor-pointer flex items-center gap-2"
                    onclick={props.on_reset.clone()}
                    disabled={*solution_state == SolutionState::Solving}
                    aria-busy={(*solution_state == SolutionState::Solving).to_string()}
                    aria-label="Reset game"
                >
                    <span>{"↻"}</span>
                    <span>{"Reset"}</span>
                </button>
            </div>

            {
                match *solution_state {
                    SolutionState::Solved(ref solution) => html! {
                        <div class="w-full max-w-md bg-green-100 border-2 border-green-500 rounded-lg p-4">
                            <div class="flex items-center gap-2 text-green-800 font-semibold mb-2">
                                <span class="text-2xl">{"✓"}</span>
                                <span>{format!("Solution found in {} operations!", solution.number_of_operations())}</span>
                            </div>
                            <details class="text-sm text-green-700">
                                <summary class="cursor-pointer hover:underline">{"View solution"}</summary>
                                <ol class="space-y-3" role="list" aria-label="Solution instructions">
                                    { for solution.instructions().iter().enumerate().filter_map(|(i, instruction)| {
                                        instruction.operation().map(|op| {
                                            let symbol = match op.operator {
                                                Operator::Add      => "+",
                                                Operator::Subtract => "−",
                                                Operator::Multiply => "×",
                                                Operator::Divide   => "÷",
                                            };
                                            let result = op.result;
                                            let numbers = instruction.state().numbers();
                                            let highlight_idx = numbers.iter().position(|&n| n == result);
                                            html! {
                                                <li class="bg-white rounded border border-green-300 p-3" role="listitem">
                                                    <div class="font-mono text-gray-800 font-semibold mb-2">
                                                        { format!("{}. {} {} {} = {}", i, op.left, symbol, op.right, result) }
                                                    </div>
                                                    <div class="flex flex-wrap gap-1">
                                                        { for numbers.iter().enumerate().map(|(idx, &n)| {
                                                            let tile_class = if Some(idx) == highlight_idx {
                                                                "bg-green-500 text-white text-sm font-semibold py-1 px-2 rounded border border-green-600"
                                                            } else {
                                                                "bg-white text-black text-sm font-semibold py-1 px-2 rounded border border-gray-400"
                                                            };
                                                            html! { <span class={tile_class}>{ n }</span> }
                                                        })}
                                                    </div>
                                                </li>
                                            }
                                        })
                                    })}
                            </ol>
                            </details>
                        </div>
                    },
                    SolutionState::NotFound => html! {
                        <div class="w-full max-w-md bg-red-100 border-2 border-red-500 rounded-lg p-4">
                            <div class="flex items-center gap-2 text-red-800 font-semibold">
                                <span class="text-2xl">{"✗"}</span>
                                <span>{"No solution found. Try a new game!"}</span>
                            </div>
                        </div>
                    },
                    _ => html! {}
                }
            }
        </div>
    }
}
