# Solver Benchmark Results

Benchmarks for the `IterativeDeepeningSolver` comparing different `StateTraversal` parent chain strategies.

All benchmarks run with [Criterion](https://bheisler.github.io/criterion.rs/) and allocation tracking via [`stats_alloc`](https://docs.rs/stats_alloc). The `stats_alloc` global allocator is active during benchmarks, so timings include its overhead — relative comparisons are valid but absolute numbers are slightly inflated vs production.

## Test Cases

| Case | Board | Target | Depth |
|------|-------|--------|-------|
| 1-step | `[1, 2, 3, 4, 5, 6]` | 12 | 1 |
| 2-step | `[1, 4, 4, 5, 6, 50]` | 350 | 2 |
| 3-step | `[1, 3, 3, 8, 9, 50]` | 410 | 3 |
| 4-step | `[2, 3, 3, 5, 6, 75]` | 277 | 4 |
| 5-step | `[1, 10, 25, 50, 75, 100]` | 831 | 5 |
| impossible | `[3, 7, 6, 2, 1, 7]` | 824 | all 6 |

## Approach 1: Baseline (Box + deep clone)

Each child `StateTraversal` owns its parent chain via `Box<StateTraversal<S>>`. When generating children from a candidate, `candidate.clone()` deep-copies the entire parent chain for every child.

### Timing

| Case | Time |
|------|------|
| 1-step | ~39 us |
| 2-step | ~506 us |
| 3-step | ~1.1 ms |
| 4-step | ~5.7 ms |
| 5-step | ~24.3 ms |
| impossible | ~8.3 ms |

### Allocations (per solve)

| Case | Allocs | Bytes allocated | Bytes freed |
|------|--------|-----------------|-------------|
| 1-step | 862 | 72,228 | 72,000 |
| 2-step | 11,608 | 830,932 | 830,648 |
| 3-step | 26,102 | 1,704,096 | 1,703,760 |
| 4-step | 135,791 | 7,622,888 | 7,622,504 |
| 5-step | 613,880 | 30,021,272 | 30,020,844 |
| impossible | 185,776 | 10,179,728 | 10,179,728 |

## Approach 2: Rc (shared ownership)

Replace `Box<StateTraversal<S>>` with `Rc<StateTraversal<S>>`. The popped candidate is wrapped in `Rc::new()` before child generation; each child receives `Rc::clone()` (cheap reference count increment) instead of a deep clone.

### Timing

| Case | Time | vs Baseline |
|------|------|-------------|
| 1-step | ~38 us | -3% |
| 2-step | ~490 us | -3% |
| 3-step | ~1.03 ms | -7% |
| 4-step | ~4.88 ms | -14% |
| 5-step | ~18.9 ms | -22% |
| impossible | ~7.0 ms | -15% |

### Allocations (per solve)

| Case | Allocs | Bytes allocated | Bytes freed | Alloc change vs Baseline |
|------|--------|-----------------|-------------|--------------------------|
| 1-step | 795 | 70,348 | 70,120 | -8% |
| 2-step | 10,494 | 803,592 | 803,308 | -3% |
| 3-step | 21,516 | 1,579,720 | 1,579,384 | -7% |
| 4-step | 95,395 | 6,491,748 | 6,491,364 | -15% |
| 5-step | 353,637 | 22,649,324 | 22,648,896 | -25% |
| impossible | 122,589 | 8,403,592 | 8,403,592 | -17% |

### Analysis

The Rc approach eliminates all deep cloning of parent chains. Improvements scale with search depth because deeper searches mean more children sharing longer parent chains. The 5-step case benefits most: 22% faster with 25% fewer allocations. The trade-off is a small per-node overhead for the reference count, which is negligible compared to the cloning savings.
