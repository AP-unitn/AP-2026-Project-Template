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

/// Placeholder until the `wind` crate reaches `Common_Code`'s `main`. It breaks the second law
/// of the wind (Main 2.4) on purpose: replace it, do not test against it.
#[must_use]
pub fn wind_at(_weather_seed: Seed, _round: Round) -> Direction {
    Direction::East
}

// ---------------------------------------------------------------- Protocol 6.1

/// A predicate over cells (Protocol 6.1).
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Clue {
    /// The Treasure is at most `radius` from `centre`.
    Within { centre: Cell, radius: u32 },
    /// The Treasure is further than `radius` from `centre`.
    Beyond { centre: Cell, radius: u32 },
    /// The Treasure's coordinate on `axis` respects `bound`.
    HalfPlane { axis: Axis, bound: Bound },
    /// The Treasure is strictly nearer `a` than `b`.
    Nearer { a: Cell, b: Cell },
}

/// One of the three hex axes.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Axis {
    /// The `q` coordinate.
    Q,
    /// The `r` coordinate.
    R,
    /// The `s` coordinate.
    S,
}

/// A floor or a ceiling on one axis.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Bound {
    /// Coordinate `>=` the value.
    AtLeast(i32),
    /// Coordinate `<=` the value.
    AtMost(i32),
}

impl Clue {
    /// Whether `cell` satisfies this Clue. Pure and total on cells of a map.
    #[must_use]
    pub fn holds(self, cell: Cell) -> bool {
        match self {
            Clue::Within { centre, radius } => cell.distance(centre) <= radius,
            Clue::Beyond { centre, radius } => cell.distance(centre) > radius,
            Clue::HalfPlane { axis, bound } => {
                let v = match axis {
                    Axis::Q => cell.q,
                    Axis::R => cell.r,
                    Axis::S => cell.s(),
                };
                match bound {
                    Bound::AtLeast(k) => v >= k,
                    Bound::AtMost(k) => v <= k,
                }
            }
            Clue::Nearer { a, b } => cell.distance(a) < cell.distance(b),
        }
    }
}

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

/// What one Ship may know (proposal P1, #110).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShipView {
    /// The round this view is valid for.
    pub round: Round,
    /// The wind of that round.
    pub wind: Direction,
    /// This Ship's cell.
    pub cell: Cell,
    /// This Ship's hull.
    pub hull: u32,
    /// Everything this Ship holds.
    pub clues: BTreeSet<Clue>,
    /// Its ally, if any.
    pub ally: Option<ShipId>,
    /// Its outgoing proposal, if any.
    pub proposal: Option<ShipId>,
    /// The Island it is moored at, if any.
    pub moored_at: Option<IslandId>,
    /// Every other Ship in a cell this Ship or its ally can see.
    pub ships: BTreeMap<ShipId, Cell>,
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
/// The trait every group's Island implements. Draft (#68). `Send` because the Chart moves
/// into the Admiralty's thread.
use std::fmt::{Display, Formatter};

/// Result of a Ship's `AskClue` action, as decided by the Island.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IslandResponse {
    /// Returns the island's original clue or a valid weakening of it.
    ///
    /// A weakening must have the same kind as the original. Every cell
    /// that satisfies the original must also satisfy the returned clue.
    Respond(Clue),
    /// The island refused to give the clue to the requesting ship due to the island's policies.
    Refused,
    /// The requesting ship has to wait at least n rounds before receiving a version of the clue.
    Wait(u32),
    /// The requesting ship has already received the full strength clue.
    NothingMore,
}

/// How an Island answers to a moor request.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoorResponse {
    /// The island approved the ship's mooring request.
    Accepted,
    /// The island rejects the request due to its policies.
    Refused,
    /// The island rejects the request due to full berth.
    NoBerth,
}

/// How an Island answers to a `repair_hull` request.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepairHullResponse {
    /// The Island approves a repair of the given number of hull points.
    /// The Chart applies the repair, capping the hull at `Params::hull_strength`
    Accepted(u32),
    /// Request rejected due to Island's own policies.
    Refused,
    /// The Ship has to wait at least n > 0 rounds to be repaired.
    Wait(u32),
}

/// Readable display intended primarily for debugging and examples.
impl Display for IslandResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IslandResponse::Respond(clue) => write!(f, "Response with clue: {clue:?}"),
            IslandResponse::Wait(round) => write!(f, "Must wait {round} rounds"),
            IslandResponse::Refused => write!(f, "Refused"),
            IslandResponse::NothingMore => write!(f, "Nothing More"),
        }
    }
}

/// Readable display intended primarily for debugging and examples.
impl Display for MoorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MoorResponse::Accepted => write!(f, "Accepted"),
            MoorResponse::NoBerth => write!(f, "NoBerth"),
            MoorResponse::Refused => write!(f, "Refused"),
        }
    }
}

/// Data supplied by the Chart when constructing a concrete Island.
///
/// The Chart determines the original Clue and provides a reproducible source
/// of Island randomness. A concrete Island is responsible for initializing its
/// own internal state from these inputs.
///
/// TODO: Confirm seed derivation/independence across Islands and how the Chart selects concrete Island implementations at construction time.
#[derive(Debug, Clone, Copy)]
pub struct IslandInit {
    /// The Island's original true Clue, generated by the Chart.
    pub clue: Clue,
    /// Deterministic seed supplied by the Chart for the Island's PRNG.
    pub seed: Seed,
    /// Game configuration; the same configuration used to generate the world.
    pub params: Params,
}
/// Defines the interface between the Chart and a stateful Island.
///
/// The Chart owns the Island and calls its methods synchronously.
/// Given the same initialization data and sequence of calls, an Island
/// must produce the same results. If it needs randomness, it must derive
/// it from [`IslandInit::seed`]. The system clock and nondeterministic
/// iteration order must not influence its decisions.
///
/// The Chart must satisfy each method's documented preconditions.
/// Violating a precondition is a caller error and must not cause a panic.
pub trait Island {
    /// Creates an Island from data supplied by the Chart.
    ///
    /// The Chart calls this method once per Island during world
    /// initialization. Each implementation initializes its own state
    /// from `init`.
    ///
    /// # Note
    /// This method requires `Self: Sized` because it constructs a concrete Island type and cannot be called through a `dyn Island` trait object.
    fn new(init: IslandInit) -> Self
    where
        Self: Sized;

    /// Registers `ship` as moored at this Island.
    ///
    /// The Chart calls this method only after validating the mooring action,
    /// including obtaining [`MoorResponse::Accepted`] from
    /// [`Island::can_moor`] for `ship`. It applies the action immediately,
    /// with no intervening change to the game state.
    fn moor(&mut self, ship: ShipId);

    /// Checks whether this Island would accept `ship` for mooring.
    ///
    /// The Chart calls this method during action validation. The method
    /// checks only the Island's policies and berth capacity. It does not
    /// modify the Island's state.
    ///
    /// Repeated calls against unchanged state must not have side effects, including consuming the Island's stateful PRNG.
    ///
    /// The returned [`MoorResponse`] describes this check; it does not record that the Ship has actually moored.
    fn can_moor(&self, ship: ShipId) -> MoorResponse; // read-only method

    /// Reports whether `ship` is currently registered as moored at this Island.
    ///
    /// This is a read-only query. If both the Chart and Island keep mooring information, their answers must stay consistent.
    ///
    /// WG TODO: The Chart already knows each Ship's mooring state; decide whether this query is necessary/useful in the trait.
    fn is_moored(&self, ship: ShipId) -> bool;

    /// Releases the berth occupied by `ship`.
    ///
    /// The Chart calls this method when an applied move takes `ship`
    /// away from this Island or when `ship` sinks while moored here.
    /// The berth must be free before the Chart validates the next action.
    ///
    /// The Island's policy determines whether unfinished waiting progress
    /// carries over to a later visit.
    fn leave_berth(&mut self, ship: ShipId);

    /// Handles a Clue request from `ship` according to the Island's policy.
    ///
    /// The Chart calls this method while applying a validated `AskClue`
    /// action, after confirming that `ship` is moored at this Island.
    ///
    /// The Island must eventually give its original, full-strength Clue
    /// to every Ship that keeps returning and waits as long as requested.
    /// Giving out the original does not consume it.
    fn ask_clue(&mut self, ship: ShipId) -> IslandResponse;

    /// Returns the number of rounds `ship` must still wait under the Island's current clue-delivery policy.
    ///
    /// - `Some(n)` indicates that `n` additional rounds of waiting are required.
    /// - `Some(0)` indicates that no further waiting is required under the current arrangement.
    /// - `None` indicates that no waiting period is currently defined for the requesting Ship.
    ///
    /// This method must not modify the Island's state.
    ///
    /// WG TODO: Confirm the exact conditions under which `None` is returned, particularly for non-moored Ships and Ships that already possess the full-strength Clue.
    fn remaining_wait(&self, ship: ShipId) -> Option<u32>;

    /// Indicates whether the Island has a Clue improvement left for `ship`.
    ///
    /// This must be `false` once the Ship has received this Island's original, full-strength Clue.
    /// Receiving only a weakened Clue does not, by itself, exhaust the Island's offer to that Ship.
    ///
    /// Read-only method.
    fn has_more_to_offer(&self, ship: ShipId) -> bool;

    /// Notifies the Island that the Chart has advanced to `round`.
    ///
    /// The Chart increments the global round first, then calls this method
    /// once per Island in ascending `IslandId` order. The game starts at
    /// round 1, so the first call receives round 2.
    ///
    /// The Chart owns the global round number.
    fn round_advance(&mut self, round: Round);

    //Optional and TBD

    /// Queries this Island's optional hull-repair service for `ship`.
    ///
    /// The Chart owns Ship hull points and applies any accepted repair, never allowing the hull to exceed `Params::hull_strength`.
    ///
    /// # Returns
    ///
    /// - `None` if the Island does not offer hull repairs.
    /// - `Some(response)` if the Island offers hull repairs.
    ///   [`RepairHullResponse::Refused`] means the Island declines this request.
    ///
    /// WG TODO: Define when this method may be called, whether its response is a quote or the actual repair decision.
    fn repair_hull(&mut self, _ship: ShipId) -> Option<RepairHullResponse> {
        None
    }

    /// Returns the mooring history this Island chooses to disclose to `requester`.
    ///
    /// The map associates each described Ship with the rounds in which it moored
    /// at this Island.
    /// Nothing else about those visits is disclosed.
    ///
    /// An Island may keep its internal history in any structure and is not
    /// required to reveal every visit.
    ///
    /// # Contract
    ///
    /// - `requester` is the Ship asking, not one of the Ships described.
    /// - `requester` must not appear as a key: the service concerns its rivals.
    /// - Each `Vec<Round>` is in ascending order.
    /// - Ships that have never moored here, or whose visits are not disclosed,
    ///   have no entry.
    /// - Iteration order is that of the `BTreeMap` (ascending `ShipId`), so the
    ///   result is deterministic.
    ///
    /// # Returns
    ///
    /// - `None`: the Island does not offer this service.
    /// - `Some(map)`: the disclosed history. An empty map is a valid response.
    ///
    /// This method must not modify the Island's state.
    ///
    /// The default implementation returns `None`, since the service is optional.
    fn ask_history(&self, _requester: ShipId) -> Option<BTreeMap<ShipId, Vec<Round>>> {
        None
    }
}