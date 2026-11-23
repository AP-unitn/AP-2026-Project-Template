# Result: undo the idiomatic changes to `crates/chart/src/rounds.rs`

## Changes

| Commit    | Message                                           |
| --------- | ------------------------------------------------- |
| `4813b9d` | revert: undo the idiomatic refactor of rounds.rs  |

`4813b9d` reverts `c829ed0`. `crates/chart/src/rounds.rs` is back to its state at `fdacf75`, and
it keeps only the three corrections of `plan-2026-11-23-002`:

- `83d300b`: `closed >= self.params.round_cap` instead of `==`, so the game also ends with
  `round_cap == 0`.
- `33a5dfd`: the winner's cell is `self.treasure`. The `?` lookup that could hide the other
  endings is gone.
- `fdacf75`: `self.round = closed.saturating_add(1)` instead of `self.round += 1`.

`plan-2026-11-23-002` is left unchanged, as the record of what was done at the time.

## Validation

- `git diff fdacf75 -- crates/chart/src/rounds.rs` after the revert: empty.
- `cargo check -p chart`: passes.
- `cargo fmt --all -- --check`: passes. The pre-commit hooks passed.

## Known limitations

- `islands_turn` is still `todo!()`, as before.
- The three clippy pedantic warnings in `rounds.rs` are back, because they came from the code the
  refactor had changed: `explicit_iter_loop`, `semicolon_if_nothing_returned` and
  `cast_possible_truncation`. Pedantic is not enabled for the `chart` crate, so a plain
  `cargo clippy` does not report them.

## Questions and answers

No clarifying questions were asked.

- **Q (user):** I don't really like this code. Go back and keep only the error corrections,
  removing the idiomatic changes.
  **A:** Reverted `c829ed0` with the new commit `4813b9d`. The three `fix:` commits are kept.
