# Result: corrections to `crates/chart/src/rounds.rs`

## Changes

Only `crates/chart/src/rounds.rs` changed. It was written entirely by the current git user. Doc
comments and the `todo!()` in `islands_turn` are untouched.

| Commit    | Message                                                    | Review point |
| --------- | ---------------------------------------------------------- | ------------ |
| `83d300b` | fix: end the game when the round cap is reached or passed  | 2            |
| `33a5dfd` | fix: take the winner's cell from the treasure              | 3            |
| `fdacf75` | fix: saturate the round counter instead of overflowing     | 4            |

- `close_round`: `self.round = closed.saturating_add(1)` instead of `self.round += 1`.
- `check_ending`:
  - `cell: self.treasure` instead of looking the winner up with `?`;
  - `closed >= self.params.round_cap` instead of `==`.

The order of the endings (declaration, then nobody afloat, then round cap) is unchanged.

The idiomatic changes of the review were also applied (`c829ed0`). At the user's request they
were reverted (`4813b9d`), so `rounds.rs` is the file as it was at `fdacf75`.

## Validation

- `cargo check -p chart` before every commit: passes.
- `cargo fmt --all -- --check` before every commit: passes. The pre-commit hooks passed on every
  commit.
- Unit tests, run in a scratch copy of the workspace and not committed:
  - **6 direct tests, against a byte-identical copy of the committed `rounds.rs`. All 6 pass.**
    - the cap is checked against the closed round: 9 gives no ending, 10 and 11 give `RoundCap`;
    - with `round_cap == 0` the game ends after round 1;
    - a declaration wins on the Treasure's cell, ahead of nobody afloat and of the round cap;
    - nobody afloat comes ahead of the round cap;
    - the ranking order is right (full-strength count descending, then `ShipId`), a sunk Ship
      keeps its recorded count, and a Clue nobody issued does not count;
    - `full_strength` counts only issued Clues.
  - **4 loop tests, with `islands_turn` replaced by `Vec::new()` in the copy only. All 4 pass.**
    - exactly `round_cap` rounds are played (10 of 10, and the counter then reads 11);
    - `close_round` after the end does nothing;
    - `round_cap == 0` ends after one round;
    - the counter saturates at `u32::MAX`.

## Known limitations and follow-ups

- `islands_turn` is still `todo!()`, so every `close_round` (and `advance_round`) panics. The user
  accepts this until the WG decides the `Island` trait. `AGENTS.md` forbids undocumented panics,
  so the panic should get a `# Panics` section when the doc comments are revisited.
- clippy pedantic reports three warnings in `rounds.rs` (`explicit_iter_loop`,
  `semicolon_if_nothing_returned`, `cast_possible_truncation`). They are not visible with a plain
  `cargo clippy`, because pedantic is not enabled for the `chart` crate. The `[lints.clippy]`
  table in the root `Cargo.toml` applies to the root package only, because it is not
  `[workspace.lints]`, and `crates/chart/src/lib.rs` has no `#![deny(clippy::pedantic)]`.
- `outcome()` in `lib.rs` is still `todo!()`. It can be
  `self.ending.as_ref().map(|e| self.describe(e))`.

## Questions and answers

No clarifying questions were asked.

- **Q (user):** Are there better, more idiomatic ways, or corrections to make, in
  `crates/chart/src/rounds.rs`? Ignore the documentation for now, and do not modify the file.
  **A:** A review in nine points, checked in a scratch copy of the workspace:
  1. `islands_turn` is `todo!()`, so every `advance_round` panics. Suggested: a no-op until
     `IslandType` implements `protocol::Island`.
  2. `closed == round_cap` never holds with `round_cap == 0`, so the game never ends. Use `>=`.
  3. The `?` on the winner lookup could hide the other two endings. Use `self.treasure`.
  4. `self.round += 1` can panic on overflow in debug builds. Use `saturating_add(1)`.
  5. to 9. Idiomatic changes: `any(ShipState::is_afloat)` instead of `afloat().is_empty()`;
     `match *ending`; `ranking` with `map` + `collect`; `full_strength` with an intersection and
     `u32::try_from`; `check_ending` as a single `if` / `else if` expression.
- **Q (user):** Apply corrections 2, 3 and 4, without the `Island` trait (the WG has not decided
  it yet), and the idiomatic changes. Write everything in `.agents` in English. The panic in
  `islands_turn` is acceptable for now.
  **A:** Commits `83d300b`, `33a5dfd`, `fdacf75` (corrections) and `c829ed0` (idiomatic changes).
- **Q (user):** I don't really like this code. Go back and keep only the error corrections.
  **A:** `c829ed0` reverted with `4813b9d`.
- **Q (user):** Since only some of the things were done in the end, update the first plan to
  match and delete the second one.
  **A:** This directory now describes only the corrections; `plan-2026-11-23-003` was deleted.
