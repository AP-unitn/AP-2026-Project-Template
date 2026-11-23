# Plan: undo the idiomatic changes to `crates/chart/src/rounds.rs`

## Goal

Keep corrections 2, 3 and 4 of `plan-2026-11-23-002` and remove the idiomatic changes 5 to 9.

## Approach

- `git revert c829ed0`. The refactor commit touches only `rounds.rs`, and no later commit touches
  that file, so the revert applies cleanly and leaves exactly the state of `fdacf75`.
- A revert commit instead of rewriting history: it is non-destructive and keeps the record of
  `plan-2026-11-23-002` consistent with the history.
- `plan-2026-11-23-002` is left as it is: it records what was done then. This directory records
  the revert.

## Validation

- `git diff fdacf75 -- crates/chart/src/rounds.rs` is empty after the revert.
- `cargo check -p chart` and `cargo fmt --all -- --check` pass.
