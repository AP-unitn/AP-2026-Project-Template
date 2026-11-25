//! Provisional protocol types for the group's crates.
//!
//! Everything protocol.tex describes and `Common_Code` does not provide yet lives here, in one
//! place. Every crate of the group imports from this crate and never straight from
//! `Common_Code`: the day `Common_Code` publishes a type, its definition here becomes a `pub use`
//! and nothing else in the workspace changes.

#![deny(unsafe_code)]
#![deny(clippy::pedantic)]

use std::collections::{BTreeMap, BTreeSet};

pub use common::geometry::{Cell, DefaultMap, Direction, Map, MapError, Terrain, visible};
pub use common::shared::{IslandId, Params, Round, Seed, ShipId, TurnToken};

use std::fmt::Debug;

// ---------------------------------------------------------------- Protocol 8 (draft, #114 and #115)

pub trait Chart: Send {
    type ValidatedAction: Debug;

    /// # Errors
    fn generate(seed: Seed, params: &Params) -> Result<Self, GenError>
    where
        Self: Sized;

    #[must_use]
    fn params(&self) -> &Params;

    #[must_use]
    fn map(&self) -> &DefaultMap;

    #[must_use]
    fn weather_seed(&self) -> Seed;

    #[must_use]
    fn round(&self) -> Round;

    /// # Errors
    fn validate(&self, ship: ShipId, action: &Action) -> Result<Self::ValidatedAction, Rejection>;

    fn apply(&mut self, action: Self::ValidatedAction) -> Vec<Report>;

    fn advance_round(&mut self) -> Vec<Report>;

    #[must_use]
    fn view_for(&self, ship: ShipId) -> Option<ShipView>;

    #[must_use]
    fn spectator_view(&self) -> SpectatorView;
    #[must_use]
    fn outcome(&self) -> Option<Outcome>;
}

/// Placeholder until the `wind` crate reaches `Common_Code`'s `main`. It breaks the second law
/// of the wind (Main 2.4) on purpose: replace it, do not test against it.
#[must_use]
pub fn wind_at(_weather_seed: Seed, _round: Round) -> Direction {
    Direction::East
}

// ---------------------------------------------------------------- Protocol 6.1

pub use common::clues::{Axis, Bound, Clue};

// ---------------------------------------------------------------- Protocol 7.1, 11.1

/// One action per round (Protocol 7.1, Main 5.3).
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Action {
    /// Sail to a neighbouring cell.
    Move { to: Cell },
    /// Moor at an adjacent Island.
    Moor { island: IslandId },
    /// Ask the Island this Ship is moored at.
    AskClue,
    /// Fire on a visible Ship within range.
    Bombard { target: ShipId },
    /// Offer an alliance.
    ProposeAlliance { to: ShipId },
    /// Accept a standing proposal.
    AcceptAlliance { proposer: ShipId },
    /// End the current alliance.
    BreakAlliance,
    /// Claim the Treasure lies in this Ship's cell.
    Declare,
    /// Do nothing.
    Hold,
}

/// Why an action was refused (Protocol 11.1, still a sketch).
#[non_exhaustive]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Rejection {
    /// Admiralty: the submission names a round that is not open.
    WrongRound { open: Round },
    /// Admiralty: the token is not the one issued.
    StaleToken,
    /// Admiralty: the outbox is over the caps.
    TooMuchMail,
    /// An action this Chart does not handle (yet). Not in the Protocol: proposal B13.
    Unsupported,
    /// Move: the target is not adjacent.
    NotANeighbour,
    /// Move: the target is outside the sea.
    OutsideArchipelago,
    /// Move: the target is land.
    CellIsLand,
    /// Move: another Ship is there.
    CellOccupied { by: ShipId },
    /// Move: the course is shut by the wind.
    AgainstTheWind,
    /// Moor: no cell of that Island is adjacent.
    NotAdjacentToIsland,
    /// Moor: every berth is taken.
    NoBerth,
    /// Moor: the Island turned the Ship away.
    IslandRefused,
    /// `AskClue`: the Ship is not moored.
    NotMoored,
    /// Bombard: the target is beyond `bombard_range`.
    OutOfRange,
    /// Bombard: the target is not in sight.
    NotVisible,
    /// The named Ship is not on the roster, or not afloat.
    NoSuchShip,
    /// Alliance: already allied.
    AlreadyAllied,
    /// Alliance: no proposal from that Ship.
    NoSuchProposal,
    /// Alliance: nobody to break with.
    NotAllied,
    /// Bombard: the target has sunk.
    TargetHasSunk,
}

// ---------------------------------------------------------------- Protocol 13.3 (draft, #65)

/// What a Ship is told happened. Draft: the WG has not settled it (#65).
#[non_exhaustive]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Event {
    /// A Ship moved into `to`.
    Moved { ship: ShipId, to: Cell },
    /// A Ship entered a rock and was left with `hull`.
    StruckRock { ship: ShipId, hull: u32 },
    /// `by` fired at `target`. Told to the victim and to whoever sees its cell.
    Fired { by: ShipId, target: ShipId },
    /// The shot landed. Told to the attacker only.
    Hit { target: ShipId, hull: u32 },
    /// The shot missed. Told to the attacker only.
    Missed { target: ShipId },
    /// A Ship went down.
    Sunk { ship: ShipId },
    /// A Ship moored at an Island.
    Moored { ship: ShipId, island: IslandId },
    /// What an Island answered. Told to the asking Ship only.
    Answered { island: IslandId, answer: IslandResponse },
    /// Public: told to everybody.
    AllianceFormed { a: ShipId, b: ShipId },
    /// Public: told to everybody.
    AllianceBroken { by: ShipId, with: ShipId },
    /// Public: told to everybody, with the cell named.
    Declared { ship: ShipId, cell: Cell },
}

/// One event and the Ships it is told to (proposal A4). The Cartographer is told every one.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Report {
    /// Who is told.
    pub to: BTreeSet<ShipId>,
    /// What happened.
    pub event: Event,
}

// ---------------------------------------------------------------- Protocol 13.4 and 13.6 (drafts)

/// What one Ship may know (Main 6.2; #130 made owned).
///
/// Only what the Ship cannot work out by itself. The round travels in `RoundOpen` and
/// `RoundResult`, the wind is `wind_at(weather_seed, round)` with the seed from `Welcome`, the
/// cells in sight follow from `visible` and `ship_position`, and the outgoing proposal is the
/// Ship's own choice.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShipView {
    /// Every Ship in a cell this Ship or its ally can see, this Ship and its ally included.
    /// A Ship out of sight, or sunk, is absent: `get` returns `None` for both.
    pub ship_position: BTreeMap<ShipId, Cell>,
    /// This Ship's hull.
    pub hull: u32,
    /// Everything this Ship holds.
    pub clues: BTreeSet<Clue>,
    /// Its ally, if any.
    pub ally: Option<ShipId>,
    /// The Island it is moored at, if any.
    pub moored_at: Option<IslandId>,
}

/// One Ship as the spectator sees it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShipSnapshot {
    /// Its cell (the last one, if sunk).
    pub cell: Cell,
    /// Its hull.
    pub hull: u32,
    /// Its hold.
    pub clues: BTreeSet<Clue>,
    /// Its ally.
    pub ally: Option<ShipId>,
    /// Its outgoing proposal.
    pub proposal: Option<ShipId>,
    /// Where it is moored.
    pub moored_at: Option<IslandId>,
    /// Whether it is still afloat.
    pub afloat: bool,
}

/// Everything (draft, #52).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SpectatorView {
    /// The open round.
    pub round: Round,
    /// Its wind.
    pub wind: Direction,
    /// The secret.
    pub treasure: Cell,
    /// Every Ship.
    pub ships: BTreeMap<ShipId, ShipSnapshot>,
    /// The Clue each Island was issued, at full strength.
    pub issued: BTreeMap<IslandId, Clue>,
}

/// How the game ended (proposal P2, #111).
#[non_exhaustive]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Outcome {
    /// Somebody declared correctly.
    Won { winner: ShipId, cell: Cell },
    /// Nobody did: round cap, or nobody afloat.
    Ranked { nobody_afloat: bool, ranking: Vec<Placing> },
}

/// One line of the ranking.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Placing {
    /// The Ship.
    pub ship: ShipId,
    /// Issued Clues it holds at full strength (recorded at sinking if sunk).
    pub full_strength: u32,
    /// Whether it sank.
    pub sunk: bool,
}

/// Why `generate` failed (#88).
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GenError {
    /// Step 2: `archipelago_radius` does not exceed `vision_range`.
    VisionRangeTooHigh,
    /// Steps 3-5 and 7: the sea is too crowded for `item`.
    CantPlace { item: GenErrItem },
    /// Step 6, after every attempt.
    WaterNotConnected,
    /// Step 9, after every attempt.
    Unsolvable,
}

/// What could not be placed.
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GenErrItem {
    /// An Island footprint.
    Island,
    /// A cliff.
    Cliff,
    /// A rock.
    Rock,
    /// The Treasure.
    Treasure,
}

// ---------------------------------------------------------------- Protocol 13.5 (draft, #68)

pub use common::islandtrait::island::{Island, IslandInit, IslandResponse, MoorResponse, RepairHullResponse};
