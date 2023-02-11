//! An implementation of the game [Turncoats](https://mildamatildagames.wordpress.com/turncoats-2/)
//! [bgg](https://boardgamegeek.com/boardgame/352238/turncoats).
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![warn(absolute_paths_not_starting_with_crate,box_pointers,elided_lifetimes_in_paths,explicit_outlives_requirements,keyword_idents,let_underscore_drop,macro_use_extern_crate,meta_variable_misuse,missing_abi,missing_copy_implementations,missing_docs,non_ascii_idents,noop_method_call,pointer_structural_match,rust_2021_incompatible_closure_captures,rust_2021_incompatible_or_patterns,rust_2021_prefixes_incompatible_syntax,rust_2021_prelude_collisions,single_use_lifetimes,trivial_casts,trivial_numeric_casts,unreachable_pub,unsafe_code,unsafe_op_in_unsafe_fn,unstable_features,unused_crate_dependencies,unused_extern_crates,unused_import_braces,unused_lifetimes,unused_macro_rules,unused_qualifications,unused_results,unused_tuple_struct_fields,variant_size_differences)]

#[doc(inline)]
pub use board::Board;
use enum_iterator::Sequence;

mod board;
/// An error that can occur when performing an action.
#[derive(Clone, Copy)]
pub struct Error {
    /// The action that caused the error.
    pub action: Action,
    /// The reason the action failed.
    pub reason: &'static str,
}

impl Error {
    const BAD_PLAYER_COUNT: &'static str = "The game supports between 2 and 5 players";
    const CANNOT_MARCH_FROM_TO: &'static str = "Cannot march between non-adjacent zones";
    const CANNOT_REMOVE_FROM_ATTACKING_FACTION: &'static str =
        "Cannot remove crew member from the attacking crew";
    const MUST_REMOVE_WHEN_ATTACKING: &'static str =
        "Must remove at least one crew member when attacking";
    const NEGOTIATION_IN_PROGRESS: &'static str = "Negotiation in progress";
    const NOT_ENOUGH_STONES_IN_ZONE: &str = "Not enough crew members in zone";
}

/// The actions available each turn.
///
/// # Errors
/// Attempting any action other than `EndNegotiation` when a negotiation is in progress will return an error.
/// Any action other than `StartNegotiation` requires the player to have at least one matching crew member in their hand.
/// See each action for the specific errors that can occur.
#[derive(Clone, Copy)]
pub enum Action {
    /// Put a crew member on the board.
    /// The player places one crew member from their hand into a zone.
    ///
    /// # Arguments
    /// * [`Crew`] - The crew that the player is placing.
    /// * [`Zone`] - The zone to place the crew member in.
    Recruit(Crew, Zone),
    /// Move crew members from one zone to another.
    /// The player places one crew member from their hand into the flag zone
    /// and moves any number of matching crew members from the source zone to the destination zone.
    ///
    /// # Arguments
    /// * [`Crew`] - The crew that the player is moving.
    /// * [`Zone`] - The zone to move crew members from.
    /// * [`Zone`] - The zone to move crew members to.
    /// * [`u8`] - The number of crew members to move.
    ///
    /// # Errors
    /// Returns an error if the zones are not adjacent. (See [`Zone`])
    /// Returns an error if there are not enough crew members in the source zone.
    March(Crew, Zone, Zone, u8),
    /// Remove crew members from a zone.
    /// The player places one crew member from their hand into the swords zone
    /// and removes non-matching crew members from a zone up to the number of matching crew members there.
    ///
    ///
    /// # Arguments
    /// * [`Crew`] - The crew that is attacking.
    /// * [`Zone`] - The zone to remove crew members from.
    /// * [`u8`] - The number of `Crew::Rogues` crew members to remove.
    /// * [`u8`] - The number of `Crew::Bullies` crew members to remove.
    /// * [`u8`] - The number of `Crew::Goons` crew members to remove.
    ///
    /// # Errors
    /// Returns an error if the amount for the attacking crew is greater than 0
    /// Returns an error if the number of attacking crew in the [`Zone`] is less than the number being removed.
    /// Returns an error if any amount given is greater than the number of corresponding crew members in the [`Zone`].
    Battle(Crew, Zone, u8, u8, u8),
    /// Start a negotiation.
    /// The player draws from the bag.
    /// This action must be followed by `EndNegotiation`.
    StartNegotiation,
    /// End a negotiation.
    /// The player that just negotiated must return a crew to the bag.
    ///
    /// If all players have negotiated in a row, the game ends.
    ///
    /// # Arguments
    /// * [`Crew`] - The crew that the player is returning to the bag.
    ///
    /// # Errors
    /// Returns an error if this action is not preceded by `StartNegotiation`.
    EndNegotiation(Crew),
}
/// The zones on the board.
#[doc = include_str!("../docs/layout.md")]
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Sequence)]
pub enum Zone {
    /// This zone starts with two `Crew::Rogues` crew members.
    Red,
    /// This zone starts with two `Crew::Bullies` crew members.
    Blue,
    /// This zone starts with two `Crew::Goons` crew members.
    Green,
    Orange,
    Yellow,
    Cyan,
    Magenta,
    Purple,
    White,
    Black,
    Gray,
}

/// The three types of crew members.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Sequence)]
pub enum Crew {
    Rogues,
    Bullies,
    Goons,
}

/// The result of a turn.
///
/// * `Board` - The board after the turn. In the case of an error, this will be the same as the board before the turn.
/// * `Result<Option<Winner>, Error>` - If the game is over, this will include the Winner. In the case of an error, this will include the error
/// see [`Action`] for the possible errors.
#[derive(Clone, Copy)]
pub struct TurnResult(pub Board, pub Result<Option<Winner>, Error>);

/// The players in the game.
/// Unused players are skipped over.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Eq, Sequence, Default)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Player {
    #[default]
    Alpha,
    Beta,
    Gamma,
    Delta,
    Epsilon,
}

/// The winner of the game.
#[derive(Clone, Copy)]
pub enum Winner {
    /// This is the player that won
    Player(Player),
    /// No player won, the game ended in a draw.
    Draw,
}

/// Starts a new game with the given number of players.
/// Returns an error if the number of players is not between 2 and 5.
///
/// # Arguments
/// * `num_players` - The number of players in the game.
///
/// # Returns
///
/// * `Ok(Board)` - The board for the game.
///
/// # Errors
/// Returns `Err(&'static str)` if the number of players is not between 2 and 5.
pub fn start_game(num_players: u8) -> Result<Board, &'static str> {
    board::Board::build(num_players)
}

/// Takes the next turn
///
/// # Arguments
/// * `board` - The current board state.
/// * `action` - The action to take.
///
/// # Returns
///
/// [`TurnResult`] - The new board state and the `Result` of the action. If the action errored, the board state will be unchanged.
///
///
#[must_use]
pub fn take_turn(board: Board, action: Action) -> TurnResult {
    board.process_action(action)
}
