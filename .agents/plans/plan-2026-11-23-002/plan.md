# Plan: corrections and idiomatic changes to `crates/chart/src/rounds.rs`

## Goal

Apply corrections 2, 3 and 4 and the idiomatic changes 5 to 9 from the review of
`crates/chart/src/rounds.rs`, without changing what the Chart reports for any reachable game state.

## Sources

- The review given earlier in the conversation (points 1 to 9, summarised in `result.md`).
- `project_docs/chart-minimale.md`: the skeleton and the contracts of `close_round`,
  `check_ending`, `ranking` and `full_strength`.
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

## Idiomatic changes

5. `check_ending`: `!self.ships.values().any(ShipState::is_afloat)` instead of
   `self.afloat().is_empty()`. It allocates nothing and stops at the first Ship afloat.
6. `describe`: `match *ending`, so the `Copy` fields bind by value and the `*winner` and `*cell`
   dereferences go away.
7. `ranking`: build the `Vec<Placing>` with `map` and `collect` instead of `for` and `push`. This
   removes the `explicit_iter_loop` and `semicolon_if_nothing_returned` pedantic lints.
8. `full_strength`:
   - match on `Status`: a sunk Ship returns the count recorded when it sank, an afloat Ship has
     its count computed. This is the contract in `chart-minimale.md`, and it lets `ranking` call
     `full_strength` for every Ship. `hitted` (`lib.rs`) is unaffected, because it calls
     `full_strength` while the target is still afloat.
   - count the intersection of the hold with the set of issued Clues, instead of
     `filter` + `any` + `**clue`. The set semantics are the same.
   - `u32::try_from(held).unwrap_or(u32::MAX)` instead of `as u32`
     (`cast_possible_truncation`).
9. `check_ending`: a single `if` / `else if` expression instead of early `return`s.

## Out of scope

- Correction 1: `islands_turn` keeps its `todo!()`. The `Island` trait has not been decided by
  the WG, and the user accepts that it panics for now.
- Doc comments: left untouched, as the user asked to ignore the documentation for now.
- `lib.rs`: `outcome()` is still `todo!()`, and clippy pedantic is not enabled for the `chart`
  crate. Both are reported, not changed.

## Commits

Only `crates/chart/src/rounds.rs` and this directory are staged. All other changes in the working
tree are left as they are.

1. `fix:` correction 2.
2. `fix:` correction 3.
3. `fix:` correction 4.
4. `refactor:` changes 5 to 9.
5. `docs:` this directory.

Before each commit: `cargo check -p chart` and `cargo fmt --all -- --check`.

## Validation

- `cargo fmt --all -- --check`.
- `cargo clippy -p chart` and `cargo clippy -p chart -- -W clippy::pedantic`: no warnings in
  `rounds.rs`.
- Unit tests in a scratch copy of the workspace, not committed:
  - `check_ending`, `describe`, `ranking` and `full_strength`, called directly against the
    committed `rounds.rs`;
  - the `close_round` loop, with `islands_turn` stubbed in the copy only.
