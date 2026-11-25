# Result

## Changes
- `crates/chart/src/lib.rs`, `do_declare`: `self.ships.get(&ship).unwrap()` + `.cell` replaced with `self.get_ship_cell(ship)`.
- `view_for` already used `get_ship_cell`. `do_move` writes the cell through `get_mut`, so it is unchanged.

## Validation
- `cargo fmt --all -- --check`: passes.
- `cargo check -p chart`: fails with E0507 in `outcome` (`self.ending?`, line 807). That code is work in progress and was not touched by this change.

## Q&A
- Q: the request was about the rest of the file, not `view_for`. A: the only other read of the cell was in `do_declare`. It has been replaced.

## Follow-up: `get_ship_cell` returns `Option<Cell>`
- The user changed `get_ship_cell` to return `Option<Cell>`.
- Rule agreed: functions called only from `apply` may `unwrap`, because `validate` has already checked the action. Every such function documents its panics with a short `# Panics` section.
- `do_declare`: now `self.get_ship_cell(ship).unwrap()`, with a `# Panics` section (panics if `ship` is not in the game; `validate` rules this out).
- The user updates the other `apply` helpers.
- `do_declare`: `unwrap()` replaced with `expect("validate guarantees the ship exists")`.
- Private helper `ship_position` renamed to `ships_in_sight`: the old name did not say it returns the Ships visible from a cell. The local `ship_position` in `view_for` keeps its name because it matches the `ShipView` field.
