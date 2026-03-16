use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use yew_countdown_solver::game::board::BoardBuilder;
use yew_countdown_solver::game::model::Game;
use yew_countdown_solver::solver::iterative_deepening::IterativeDeepeningSolver;
use yew_countdown_solver::solver::traits::Solver;

/// Helper to build a Game from a target and a slice of numbers.
fn make_game(target: u32, numbers: &[u32]) -> Game {
    let mut builder = BoardBuilder::new();
    for &n in numbers {
        builder = builder.add_number(n).unwrap();
    }
    let board = builder.build().unwrap();
    Game::new(board, target).unwrap()
}

/// Benchmark the solver across games of increasing difficulty.
///
/// Each case is named by the number of operations in the optimal solution
/// (or "impossible" for the unsolvable game). This lets us see how each
/// depth level is affected by a change.
fn solver_benchmark(c: &mut Criterion) {
    let cases: Vec<(&str, Game)> = vec![
        ("1-step", make_game(12, &[1, 2, 3, 4, 5, 6])),
        ("2-step", make_game(350, &[1, 4, 4, 5, 6, 50])),
        ("3-step", make_game(410, &[1, 3, 3, 8, 9, 50])),
        ("4-step", make_game(277, &[2, 3, 3, 5, 6, 75])),
        ("5-step", make_game(831, &[1, 10, 25, 50, 75, 100])),
        ("impossible", make_game(824, &[3, 7, 6, 2, 1, 7])),
    ];

    let mut group = c.benchmark_group("solver");

    for (label, game) in &cases {
        group.bench_with_input(BenchmarkId::new("solve", label), game, |b, game| {
            b.iter(|| {
                let solver = IterativeDeepeningSolver::new(game);
                solver.solve()
            });
        });
    }

    group.finish();
}

criterion_group!(benches, solver_benchmark);
criterion_main!(benches);
