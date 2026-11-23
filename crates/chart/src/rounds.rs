//! This file wrap all the chart function between two rounds in a game.

use protocol::{Clue, Outcome, Placing, Report, Round};
use std::cmp::Reverse;
use std::collections::BTreeSet;

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
        todo!() //waiting an island trait
    }

    /// check if the game is over
    fn check_ending(&self, closed: Round) -> Option<Ending> {
        if let Some(winner) = self.winner {
            Some(Ending::Declared {
                winner,
                cell: self.treasure,
            })
        } else if !self.ships.values().any(ShipState::is_afloat) {
            Some(Ending::NobodyAfloat)
        } else if closed >= self.params.round_cap {
            Some(Ending::RoundCap)
        } else {
            None
        }
    }

    /// describe the ending retuning an `Outcome`
    pub(crate) fn describe(&self, ending: &Ending) -> Outcome {
        match *ending {
            Ending::Declared { winner, cell } => Outcome::Won { winner, cell },
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
        let mut ranking: Vec<Placing> = self
            .ships
            .iter()
            .map(|(&ship, state)| Placing {
                ship,
                full_strength: self.full_strength(state),
                sunk: !state.is_afloat(),
            })
            .collect();
        ranking.sort_by_key(|p| (Reverse(p.full_strength), p.ship));
        ranking
    }

    /// Return the score of a ship after it was sunk
    pub(crate) fn full_strength(&self, ship: &ShipState) -> u32 {
        match ship.status {
            Status::Sunk { full_strength } => full_strength,
            Status::Afloat => {
                let issued: BTreeSet<Clue> = self.islands.values().map(|slot| slot.clue).collect();
                let held = ship.clues.intersection(&issued).count();
                u32::try_from(held).unwrap_or(u32::MAX)
            }
        }
    }
}
