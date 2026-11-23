# Plan: corrections to `crates/chart/src/rounds.rs`

## Goal

Apply corrections 2, 3 and 4 from the review of `crates/chart/src/rounds.rs`. They do not change
what the Chart reports for any game state reachable with valid parameters.

## Sources

- The review given earlier in the conversation (summarised in `result.md`).
- `project_docs/chart-minimale.md`: the skeleton and the contracts of `close_round` and
  `check_ending`.
- Protocol 8.2: `advance_round` increments the round, gives every Island its turn, then decides
  whether the game has ended.
- Proposal A11 in `project_docs/results-2026-11-13-errori-mancanti.md`: exactly `round_cap` rounds
  are played; the cap is checked against the round that has just been closed.
- `AGENTS.md`: no undocumented panics, atomic conventional commits, `cargo fmt --all` before
  committing.

## Corrections

2. `check_ending`: compare `closed >= self.params.round_cap` instead of `==`. For every
   `round_cap >= 1` the game ends on the same round as before, because `closed` is the round just
   played and not the incremented counter (the A11 pitfall). With `round_cap == 0`, `==` never
   holds and the game never ends; `>=` ends it after the first round.
3. `check_ending`: build `Ending::Declared` with `cell: self.treasure` instead of
   `self.ships.get(&winner)?.cell`. A correct declaration means the declaring Ship stands on the
   Treasure, so the lookup is not needed. Its `?` would also turn a failed lookup into "the game
   is not over", and skip the nobody-afloat and round-cap checks.
4. `close_round`: `self.round = closed.saturating_add(1)` instead of `self.round += 1`, which
   panics on overflow in debug builds.

## Out of scope

- Correction 1: `islands_turn` keeps its `todo!()`. The `Island` trait has not been decided by
  the WG, and the user accepts that it panics for now.
- The idiomatic changes of the review (points 5 to 9).
- Doc comments: left untouched, as the user asked to ignore the documentation for now.
- `lib.rs`: `outcome()` is still `todo!()`, and clippy pedantic is not enabled for the `chart`
  crate. Both are reported, not changed.

## Commits

Only `crates/chart/src/rounds.rs` and this directory are staged. All other changes in the working
tree are left as they are.

1. `fix:` correction 2.
2. `fix:` correction 3.
3. `fix:` correction 4.
4. `docs:` this directory.

Before each commit: `cargo check -p chart` and `cargo fmt --all -- --check`.

## Validation

- `cargo fmt --all -- --check` and `cargo check -p chart`.
- Unit tests in a scratch copy of the workspace, not committed:
  - `check_ending`, `describe`, `ranking` and `full_strength`, called directly against the
    committed `rounds.rs`;
  - the `close_round` loop, with `islands_turn` stubbed in the copy only.
