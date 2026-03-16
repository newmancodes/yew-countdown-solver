use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, BenchmarkId, Criterion,
};
use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};
use std::alloc::System;
use yew_countdown_solver::game::board::BoardBuilder;
use yew_countdown_solver::game::model::Game;
use yew_countdown_solver::solver::iterative_deepening::IterativeDeepeningSolver;
use yew_countdown_solver::solver::traits::Solver;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

/// Helper to build a Game from a target and a slice of numbers.
fn make_game(target: u32, numbers: &[u32]) -> Game {
    let mut builder = BoardBuilder::new();
    for &n in numbers {
        builder = builder.add_number(n).unwrap();
    }
    let board = builder.build().unwrap();
    Game::new(board, target).unwrap()
}

fn test_cases() -> Vec<(&'static str, Game)> {
    vec![
        ("1-step", make_game(12, &[1, 2, 3, 4, 5, 6])),
        ("2-step", make_game(350, &[1, 4, 4, 5, 6, 50])),
        ("3-step", make_game(410, &[1, 3, 3, 8, 9, 50])),
        ("4-step", make_game(277, &[2, 3, 3, 5, 6, 75])),
        ("5-step", make_game(831, &[1, 10, 25, 50, 75, 100])),
        ("impossible", make_game(824, &[3, 7, 6, 2, 1, 7])),
    ]
}

/// Print per-solve allocation statistics for each test case.
///
/// This runs alongside the Criterion timing benchmarks but uses
/// stats_alloc to measure heap allocations rather than wall-clock time.
/// Output goes to stdout so it appears in `cargo bench` output.
fn print_allocation_stats(cases: &[(&str, Game)]) {
    println!();
    println!("=== Allocation Statistics (per solve) ===");
    println!(
        "{:<12} {:>12} {:>16} {:>16}",
        "Case", "Allocs", "Bytes alloc'd", "Bytes freed"
    );
    println!("{}", "-".repeat(60));

    for (label, game) in cases {
        let reg = Region::new(&GLOBAL);
        let solver = IterativeDeepeningSolver::new(game);
        let _solution = solver.solve();
        let stats = reg.change();

        println!(
            "{:<12} {:>12} {:>16} {:>16}",
            label, stats.allocations, stats.bytes_allocated, stats.bytes_deallocated,
        );
    }

    println!("{}", "-".repeat(60));
    println!();
}

/// Wrapper that runs timing benchmarks then prints allocation stats.
fn solver_benchmark_with_allocations(c: &mut Criterion) {
    let cases = test_cases();
    solver_timing_benchmarks(c, &cases);
    print_allocation_stats(&cases);
}

fn solver_timing_benchmarks(c: &mut Criterion, cases: &[(&str, Game)]) {
    let mut group: BenchmarkGroup<WallTime> = c.benchmark_group("solver");

    for (label, game) in cases {
        group.bench_with_input(BenchmarkId::new("solve", label), game, |b, game| {
            b.iter(|| {
                let solver = IterativeDeepeningSolver::new(game);
                solver.solve()
            });
        });
    }

    group.finish();
}

criterion_group!(benches, solver_benchmark_with_allocations);
criterion_main!(benches);
