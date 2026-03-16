use gloo_timers::callback::Timeout;
use yew::prelude::*;
use yew_countdown_solver::game::board::Board;
use yew_countdown_solver::game::model::Game;
use yew_countdown_solver::solver::iterative_deepening::IterativeDeepeningSolver;
use yew_countdown_solver::solver::traits::{Instruction, Operator, Solver};

#[derive(Properties, PartialEq)]
pub struct GameBoardProps {
    pub game: Game,
    pub on_reset: Callback<MouseEvent>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SolutionState {
    NotAttempted,
    Competing(u8), // seconds remaining: N → 0
    Solving,
    Solved {
        solution: yew_countdown_solver::solver::traits::Solution<Game, Board>,
        elapsed_ms: f64,
    },
    NotFound,
}

/// Read the compete timer duration from localStorage.
///
/// If the key `OPTS_DEV_FAST_COMPETE` is set to `"true"`, returns 2 seconds
/// (for fast E2E testing). Otherwise returns the standard 30 seconds.
fn get_compete_duration() -> u8 {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(val)) = storage.get_item("OPTS_DEV_FAST_COMPETE") {
                if val == "true" {
                    return 2;
                }
            }
        }
    }
    30
}

#[component]
pub fn GameBoard(props: &GameBoardProps) -> Html {
    let solution_state = use_state(|| SolutionState::NotAttempted);
    let game = props.game.clone();

    // --- State-driven effect for countdown ticks and deferred solving ---
    // Keyed on the full SolutionState so each tick (Competing(n) -> Competing(n-1))
    // creates a fresh 1-second Timeout with up-to-date state, avoiding stale closures.
    // Also handles the Solving state: yields to the browser event loop via a 0ms
    // timeout so the "Solving..." UI actually renders before the synchronous solver
    // blocks the main thread. Both Solve and Compete converge here.
    {
        let solution_state = solution_state.clone();
        let game = game.clone();
        use_effect_with((*solution_state).clone(), move |state: &SolutionState| {
            // Store an optional Timeout handle for cleanup.
            // Only the countdown tick timeout needs cancellation on cleanup;
            // the 0ms solve-trigger timeout is forgotten (fire-and-forget).
            let mut cleanup_handle: Option<Timeout> = None;

            match *state {
                SolutionState::Competing(n) => {
                    let solution_state = solution_state.clone();
                    if n > 0 {
                        // Schedule the next tick in 1 second
                        let timeout = Timeout::new(1_000, move || {
                            solution_state.set(SolutionState::Competing(n - 1));
                        });
                        cleanup_handle = Some(timeout);
                    } else {
                        // Timer reached 0: transition to Solving
                        solution_state.set(SolutionState::Solving);
                    }
                }
                SolutionState::Solving => {
                    // Yield to the browser event loop via a 0ms timeout so the
                    // "Solving..." UI renders before the solver blocks the thread.
                    let solution_state = solution_state.clone();
                    let game = game.clone();
                    let timeout = Timeout::new(0, move || {
                        let performance = web_sys::window().and_then(|w| w.performance());
                        let start = performance.as_ref().map(|p| p.now());

                        let solver = IterativeDeepeningSolver::new(&game);
                        if let Some(solution) = solver.solve() {
                            let elapsed_ms = match (start, performance.as_ref()) {
                                (Some(s), Some(p)) => p.now() - s,
                                _ => 0.0,
                            };
                            tracing::info!(
                                "Found solution for game {:?} in {} operations ({:.1}ms)",
                                game,
                                solution.number_of_operations(),
                                elapsed_ms,
                            );
                            solution_state.set(SolutionState::Solved {
                                solution,
                                elapsed_ms,
                            });
                        } else {
                            tracing::info!("No solution found for game {:?}", game);
                            solution_state.set(SolutionState::NotFound);
                        }
                    });
                    // Forget so it fires even after cleanup runs.
                    timeout.forget();
                }
                _ => {}
            }

            // Single cleanup closure: drops the countdown timeout if one was set.
            move || drop(cleanup_handle)
        });
    }

    // --- Solve button click handler ---
    // Just transitions to Solving state; the effect above handles the actual work.
    let on_solve_click = {
        let solution_state = solution_state.clone();

        Callback::from(move |_| {
            solution_state.set(SolutionState::Solving);
        })
    };

    // --- Compete button click handler ---
    let on_compete_click = {
        let solution_state = solution_state.clone();
        Callback::from(move |_| {
            let duration = get_compete_duration();
            solution_state.set(SolutionState::Competing(duration));
        })
    };

    // --- Disable logic ---
    let is_not_attempted = *solution_state == SolutionState::NotAttempted;
    let is_busy = matches!(
        *solution_state,
        SolutionState::Solving | SolutionState::Competing(_)
    );

    html! {
        <div class="flex flex-col items-center gap-6 p-4">
            <section aria-label="Game board" class="w-full max-w-md">
                <div class="bg-blue-600 text-white text-2xl font-bold text-center py-4 px-6 border-2 border-gray-600 rounded-lg mb-4">
                    <span aria-label="Target number">{ game.target() }</span>
                </div>

                <div class="grid grid-cols-3 sm:grid-cols-6 gap-3" role="list" aria-label="Available numbers">
                    { for game.board().numbers().iter().map(|number| {
                        html! {
                            <div role="listitem" class="bg-white text-black text-xl font-semibold flex items-center justify-center py-4 px-6 border-2 border-gray-400 rounded-lg">
                                { *number }
                            </div>
                        }
                    })}
                </div>
            </section>

            // Timer display (only visible during Competing state)
            {
                if let SolutionState::Competing(seconds) = *solution_state {
                    html! {
                        <div class="w-full max-w-md bg-yellow-100 border-2 border-yellow-500 rounded-lg p-4 text-center">
                            <div class="text-yellow-800 font-semibold mb-1">{"Time remaining"}</div>
                            <div class="text-5xl font-bold text-yellow-900" aria-label="Time remaining">
                                { seconds }
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }
            }

            <div class="flex flex-wrap gap-3 justify-center">
                <button
                    class="bg-gray-500 hover:bg-gray-700 disabled:bg-gray-400 disabled:cursor-not-allowed text-white font-bold py-3 px-8 rounded-lg shadow-md transition-colors duration-200 cursor-pointer flex items-center gap-2 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-gray-600"
                    onclick={props.on_reset.clone()}
                    disabled={is_busy}
                    aria-busy={is_busy.to_string()}
                    aria-label="Start new game"
                >
                    <span aria-hidden="true">{"↻"}</span>
                    <span>{"New Game"}</span>
                </button>

                <button
                    class="bg-blue-500 hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed text-white font-bold py-3 px-8 rounded-lg shadow-md transition-colors duration-200 cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-blue-600"
                    onclick={on_solve_click}
                    disabled={!is_not_attempted}
                    aria-busy={(!is_not_attempted).to_string()}
                    aria-label="Solve game"
                >
                    { if *solution_state == SolutionState::Solving { "Solving..." } else { "Solve" } }
                </button>

                <button
                    class="bg-orange-500 hover:bg-orange-700 disabled:bg-gray-400 disabled:cursor-not-allowed text-white font-bold py-3 px-8 rounded-lg shadow-md transition-colors duration-200 cursor-pointer focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-orange-600"
                    onclick={on_compete_click}
                    disabled={!is_not_attempted}
                    aria-label="Compete"
                >
                    {"Compete"}
                </button>
            </div>

            {
                match *solution_state {
                    SolutionState::Solved { ref solution, elapsed_ms } => html! {
                        <div class="w-full max-w-md bg-green-100 border-2 border-green-500 rounded-lg p-4">
                            <div class="flex items-center gap-2 text-green-800 font-semibold mb-2">
                                <span aria-hidden="true" class="text-2xl">{"✓"}</span>
                                <span>{format!("Solution found in {:.1}ms with {} operations!", elapsed_ms, solution.number_of_operations())}</span>
                            </div>
                            <ol class="space-y-3 text-sm text-green-700" role="list" aria-label="Solution instructions">
                                { for solution.instructions().iter().enumerate().filter_map(|(i, instruction): (usize, &Instruction<Board>)| {
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
                                                            "bg-green-700 text-white text-sm font-semibold py-1 px-2 rounded border border-green-800"
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
                        </div>
                    },
                    SolutionState::NotFound => html! {
                        <div class="w-full max-w-md bg-red-100 border-2 border-red-500 rounded-lg p-4">
                            <div class="flex items-center gap-2 text-red-800 font-semibold">
                                <span aria-hidden="true" class="text-2xl">{"✗"}</span>
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
