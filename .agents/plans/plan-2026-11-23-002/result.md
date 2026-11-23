# Result: corrections and idiomatic changes to `crates/chart/src/rounds.rs`

## Changes

Only `crates/chart/src/rounds.rs` changed. It was written entirely by the current git user. Doc
comments and the `todo!()` in `islands_turn` are untouched.

| Commit    | Message                                                    | Review point |
| --------- | ---------------------------------------------------------- | ------------ |
| `83d300b` | fix: end the game when the round cap is reached or passed  | 2            |
| `33a5dfd` | fix: take the winner's cell from the treasure              | 3            |
| `fdacf75` | fix: saturate the round counter instead of overflowing     | 4            |
| `c829ed0` | refactor: make round closing and ranking more idiomatic    | 5 to 9       |

- `close_round`: `self.round = closed.saturating_add(1)` instead of `self.round += 1`.
- `check_ending`:
  - a single `if` / `else if` expression;
  - `cell: self.treasure` instead of looking the winner up with `?`;
  - `!self.ships.values().any(ShipState::is_afloat)` instead of `self.afloat().is_empty()`;
  - `closed >= self.params.round_cap` instead of `==`.
- `describe`: `match *ending`, with no dereferences in the arms.
- `ranking`: built with `map` and `collect`. It calls `full_strength` for every Ship and takes
  `sunk` from `!state.is_afloat()`.
- `full_strength`:
  - matches on `Status`: a sunk Ship returns its recorded count;
  - a Ship afloat counts the intersection of its hold with the set of issued Clues;
  - the count is converted with `u32::try_from(held).unwrap_or(u32::MAX)`.

The order of the endings (declaration, then nobody afloat, then round cap) is unchanged. So is
the ranking (full-strength count descending, then `ShipId` ascending) for every state.

## Validation

- `cargo check -p chart` before every commit: passes.
- `cargo fmt --all -- --check` before every commit: passes. The pre-commit hooks passed on every
  commit.
- `cargo clippy -p chart`: no warnings in `rounds.rs`.
- `cargo clippy -p chart -- -W clippy::pedantic`: no warnings in `rounds.rs`. Before the
  refactor it reported `explicit_iter_loop`, `semicolon_if_nothing_returned` and
  `cast_possible_truncation`.
- Unit tests, run in a scratch copy of the workspace and not committed:
  - **7 direct tests, against a byte-identical copy of the committed `rounds.rs`. All 7 pass.**
    - the cap is checked against the closed round: 9 gives no ending, 10 and 11 give `RoundCap`;
    - with `round_cap == 0` the game ends after round 1;
    - a declaration wins on the Treasure's cell, ahead of nobody afloat and of the round cap;
    - nobody afloat comes ahead of the round cap;
    - the ranking order is right, a sunk Ship keeps its recorded count, and a Clue nobody issued
      does not count;
    - `full_strength` counts only issued Clues;
    - for a sunk Ship, `full_strength` returns the recorded count.
  - **4 loop tests, with `islands_turn` replaced by `Vec::new()` in the copy only. All 11 tests
    pass.**
    - exactly `round_cap` rounds are played (10 of 10, and the counter then reads 11);
    - `close_round` after the end does nothing;
    - `round_cap == 0` ends after one round;
    - the counter saturates at `u32::MAX`.

## Known limitations and follow-ups

- `islands_turn` is still `todo!()`, so every `close_round` (and `advance_round`) panics. The user
  accepts this until the WG decides the `Island` trait. `AGENTS.md` forbids undocumented panics,
  so the panic should get a `# Panics` section when the doc comments are revisited.
- Doc comments were left as they were, as asked. The comment on `full_strength` ("Return the
  score of a ship after it was sunk") does not describe its contract any more: the recorded count
  for a sunk Ship, the computed count for a Ship afloat.
- The issued Clues are collected again on every `full_strength` call. They are fixed after
  generation and could be stored once; `SpectatorView::issued` will need them too.
- `DefaultChart::afloat()` is no longer used inside the crate. It stays as public API (proposal
  A9).
- Outside `rounds.rs`, reported and not changed:
  - `outcome()` in `lib.rs` is still `todo!()`. It can be
    `self.ending.as_ref().map(|e| self.describe(e))`.
  - clippy pedantic is not enabled for the `chart` crate. The `[lints.clippy]` table in the root
    `Cargo.toml` applies to the root package only, because it is not `[workspace.lints]`, and
    `crates/chart/src/lib.rs` has no `#![deny(clippy::pedantic)]`.

## Questions and answers

No clarifying questions were asked.

- **Q (user):** Are there better, more idiomatic ways, or corrections to make, in
  `crates/chart/src/rounds.rs`? Ignore the documentation for now, and do not modify the file.
  **A:** A review in nine points, checked in a scratch copy of the workspace:
  1. `islands_turn` is `todo!()`, so every `advance_round` panics. With the original code, all six
     scratch tests panicked at `rounds.rs:25`. Suggested: a no-op until `IslandType` implements
     `protocol::Island`, then a loop over `self.islands.values_mut()` calling `end_of_round`.
  2. `closed == round_cap` never holds with `round_cap == 0`, so the game never ends (verified).
     Use `>=`: same behaviour for every cap >= 1, because `closed` is not the incremented counter
     (proposal A11).
  3. The `?` on the winner lookup could hide the other two endings. Use `self.treasure`.
  4. `self.round += 1` can panic on overflow in debug builds. Use `saturating_add(1)`.
  5. `self.afloat().is_empty()` allocates a set. Use `any(ShipState::is_afloat)`.
  6. `describe` can `match *ending`.
  7. `ranking` can use `map` + `collect`. Moving the `Status` match into `full_strength` makes it
     trivial.
  8. `full_strength`: use `u32::try_from` instead of `as`, and count the intersection with the
     issued Clues.
  9. `check_ending` can be a single `if` / `else if` expression.

  Also reported: `outcome()` in `lib.rs` is still `todo!()`, and clippy pedantic is not enabled
  for the `chart` crate. With only the `todo!()` replaced, the original code passed 5 of the 6
  scratch tests; the one with `round_cap == 0` failed. The proposed version passed all six.
- **Q (user):** Apply corrections 2, 3 and 4, without the `Island` trait (the WG has not decided
  it yet), and the idiomatic changes. Write everything in `.agents` in English. The panic in
  `islands_turn` is acceptable for now.
  **A:** The four commits above, and this directory.
