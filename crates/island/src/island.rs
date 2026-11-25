#![deny(unsafe_code)]
#![allow(dead_code, unused_variables, clippy::missing_errors_doc)]

use protocol::IslandResponse::NothingMore;
use protocol::{
    Clue, Island, IslandInit, IslandResponse, MoorResponse, Params, RepairHullResponse, Round, Seed, ShipId,
};
use std::collections::BTreeMap;

pub struct DefaultIsland {
    full_clue: Clue,
    seed: Seed,
    berths: Option<u32>,
    params: Params,
    current_round: Round,

    ship_moored_now: BTreeMap<ShipId, Round>,
    last_answers: BTreeMap<ShipId, IslandResponse>,
}

impl DefaultIsland {
    fn new(init: IslandInit) -> Self {
        DefaultIsland {
            full_clue: init.clue,
            seed: init.seed,
            params: init.params,
            current_round: 1,
            ship_moored_now: BTreeMap::new(),
            last_answers: BTreeMap::new(),
            berths: Some(6),
        }
    }

    fn moor(&mut self, ship: ShipId) {
        if !self.ship_moored_now.contains_key(&ship) {
            self.ship_moored_now.insert(ship, self.current_round);
        }
    }

    fn can_moor(&self, ship: ShipId) -> MoorResponse {
        if self.ship_moored_now.contains_key(&ship) {
            MoorResponse::Accepted
        } else {
            match self.berths {
                Some(berths) => {
                    if (self.ship_moored_now.len() as u32) < berths {
                        MoorResponse::Accepted
                    } else {
                        MoorResponse::NoBerth
                    }
                }
                None => MoorResponse::Accepted,
            }
        }
    }

    fn is_moored(&self, ship: ShipId) -> bool {
        self.ship_moored_now.contains_key(&ship)
    }

    fn leave_berth(&mut self, ship: ShipId) {
        self.ship_moored_now.remove(&ship);
    }

    fn ask_clue(&mut self, ship: ShipId) -> IslandResponse {
        if self.last_answers.get(&ship) == Some(&IslandResponse::Respond(self.full_clue)) {
            self.last_answers.insert(ship, NothingMore);
            NothingMore
        } else if self.last_answers.get(&ship) == Some(&NothingMore) {
            NothingMore
        } else if self.last_answers.get(&ship) == None {
            self.last_answers.insert(ship, IslandResponse::Respond(self.full_clue));
            IslandResponse::Respond(self.full_clue)
        } else {
            self.last_answers.insert(ship, IslandResponse::Respond(self.full_clue));
            IslandResponse::Respond(self.full_clue)
        }
    }

    fn remaining_wait(&self, ship: ShipId) -> Option<u32> {
        if !self.ship_moored_now.contains_key(&ship)
            || self.last_answers.get(&ship) == Some(&IslandResponse::Respond(self.full_clue))
        {
            None
        } else {
            Some(0)
        }
    }

    fn has_more_to_offer(&self, ship: ShipId) -> bool {
        !(self.last_answers.get(&ship) == Some(&NothingMore)
            || self.last_answers.get(&ship) == Some(&IslandResponse::Respond(self.full_clue)))
    }

    fn round_advance(&mut self, round: Round) {
        self.current_round = round;
    }
    fn repair_hull(&mut self, _ship: ShipId) -> Option<RepairHullResponse> {
        None
    }

    fn ask_history(&self, _requester: ShipId) -> Option<BTreeMap<ShipId, Vec<Round>>> {
        None
    }
}

impl Island for DefaultIsland {
    fn new(init: IslandInit) -> Self
    where
        Self: Sized,
    {
        DefaultIsland::new(init)
    }

    fn moor(&mut self, ship: ShipId) {
        self.moor(ship);
    }

    fn can_moor(&self, ship: ShipId) -> MoorResponse {
        self.can_moor(ship)
    }

    fn is_moored(&self, ship: ShipId) -> bool {
        self.is_moored(ship)
    }

    fn leave_berth(&mut self, ship: ShipId) {
        self.leave_berth(ship);
    }

    fn ask_clue(&mut self, ship: ShipId) -> IslandResponse {
        self.ask_clue(ship)
    }

    fn remaining_wait(&self, ship: ShipId) -> Option<u32> {
        self.remaining_wait(ship)
    }

    fn has_more_to_offer(&self, ship: ShipId) -> bool {
        self.has_more_to_offer(ship)
    }

    fn round_advance(&mut self, round: Round) {
        self.round_advance(round);
    }

    fn repair_hull(&mut self, _ship: ShipId) -> Option<RepairHullResponse> {
        self.repair_hull(_ship)
    }

    fn ask_history(&self, _requester: ShipId) -> Option<BTreeMap<ShipId, Vec<Round>>> {
        self.ask_history(_requester)
    }
}
