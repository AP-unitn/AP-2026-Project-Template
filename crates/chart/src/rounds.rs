//! This file wrap all the chart function between two rounds in a game.

use protocol::{Outcome, Placing, Report, Round};
use std::cmp::Reverse;

use crate::DefaultChart;
use crate::state::{Ending, ShipState, Status};

impl DefaultChart {
    /// advances the rounds
    pub(crate) fn close_round(&mut self) -> Vec<Report> {
        if self.ending.is_some() {
            return Vec::new();
        }
        let closed = self.round;
        self.round = closed.saturating_add(1);

        let reports = self.islands_turn();
        self.ending = self.check_ending(closed);
        reports
    }

    /// call every island at the end of the round
    fn islands_turn(&mut self) -> Vec<Report> {
        self.islands
            .values_mut()
            .for_each(|island_slot| island_slot.island.round_advance(self.round));
        Vec::new()
    }

    /// check if the game is over
    fn check_ending(&self, closed: Round) -> Option<Ending> {
        if let Some(winner) = self.winner {
            return Some(Ending::Declared {
                winner,
                cell: self.treasure,
            });
        }
        if self.afloat().is_empty() {
            return Some(Ending::NobodyAfloat);
        }
        if closed >= self.params.round_cap {
            return Some(Ending::RoundCap);
        }
        None
    }

    /// describe the ending retuning an `Outcome`
    pub(crate) fn describe(&self, ending: &Ending) -> Outcome {
        match ending {
            Ending::Declared { winner, cell } => Outcome::Won {
                winner: *winner,
                cell: *cell,
            },
            Ending::NobodyAfloat => Outcome::Ranked {
                nobody_afloat: true,
                ranking: self.ranking(),
            },
            Ending::RoundCap => Outcome::Ranked {
                nobody_afloat: false,
                ranking: self.ranking(),
            },
        }
    }

    /// Build the ranking
    fn ranking(&self) -> Vec<Placing> {
        let mut rank: Vec<Placing> = Vec::new();
        for (ship_id, ship_state) in self.ships.iter() {
            match ship_state.status {
                Status::Sunk { full_strength } => rank.push(Placing {
                    ship: *ship_id,
                    full_strength,
                    sunk: true,
                }),
                Status::Afloat => {
                    let full_strength = self.full_strength(ship_state);
                    rank.push(Placing {
                        ship: *ship_id,
                        full_strength,
                        sunk: false,
                    })
                }
            }
        }
        rank.sort_by_key(|p| (Reverse(p.full_strength), p.ship));
        rank
    }

    /// Return the score of a ship after it was sunk
    pub(crate) fn full_strength(&self, ship: &ShipState) -> u32 {
        ship.clues
            .iter()
            .filter(|clue| self.islands.values().any(|island_slot| island_slot.clue == **clue))
            .count() as u32
    }
}
