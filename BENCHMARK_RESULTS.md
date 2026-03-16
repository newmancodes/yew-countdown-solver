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

## Approach 3: Arena (typed-arena) -- Winner

Replace individual heap allocations with a per-depth-iteration `typed_arena::Arena`. All `StateTraversal` nodes are bump-allocated into the arena; children reference parents via `&'arena StateTraversal` borrows. The entire arena is freed in one shot when the depth iteration ends.

### Timing

| Case | Time | vs Baseline | vs Rc |
|------|------|-------------|-------|
| 1-step | ~38 us | -3% | ~0% |
| 2-step | ~482 us | -5% | -2% |
| 3-step | ~1.02 ms | -7% | -1% |
| 4-step | ~4.79 ms | -16% | -2% |
| 5-step | ~18.5 ms | -24% | -2% |
| impossible | ~6.9 ms | -17% | -2% |

### Allocations (per solve)

| Case | Allocs | Bytes allocated | Bytes freed | Alloc change vs Baseline |
|------|--------|-----------------|-------------|--------------------------|
| 1-step | 780 | 70,540 | 70,312 | -10% |
| 2-step | 10,098 | 809,080 | 808,796 | -13% |
| 3-step | 20,407 | 1,600,264 | 1,599,928 | -22% |
| 4-step | 87,871 | 6,542,836 | 6,542,452 | -35% |
| 5-step | 313,503 | 22,701,036 | 22,700,608 | -49% |
| impossible | 111,762 | 8,367,144 | 8,367,144 | -40% |

### Analysis

The arena approach wins on both timing and allocations. Key observations:

1. **Allocation count reduction scales dramatically with depth** — at 5-step depth, arena uses 49% fewer allocations than baseline. This is because the arena batches many small allocations into large chunks, and individual `StateTraversal` nodes no longer need their own heap allocation for the parent pointer.

2. **Timing improvement over Rc is modest (~2%)** — the main win (eliminating deep cloning) was already captured by Rc. The arena's additional advantage comes from cheaper allocation (bump pointer vs `malloc`) and cheaper deallocation (bulk free vs individual `drop`).

3. **Bytes allocated is slightly higher than Rc for some cases** — the arena pre-allocates chunks, so there is some unused capacity within each chunk. This is a space-time trade-off: slightly more memory reserved, but faster allocation and deallocation.

4. **Code complexity is comparable to Rc** — the arena version uses `&'arena StateTraversal` borrows instead of `Rc<StateTraversal>`. The arena is created per depth iteration and naturally scopes the lifetime of all nodes.
