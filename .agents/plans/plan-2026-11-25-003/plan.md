# Plan

- Find every read of a ship's cell in `crates/chart/src/lib.rs` done through `self.ships.get(..).unwrap()`.
- Replace it with `self.get_ship_cell(ship)`.
- Writes (`do_move`, via `get_mut`) stay as they are: the getter returns a copy.
- Run `cargo fmt --all`.
