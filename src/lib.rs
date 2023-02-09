use board::Board;
use enum_iterator::Sequence;

mod bag;
mod board;

#[derive(Debug)]
pub struct Error {
    pub action: Action,
    pub reason: &'static str,
}

impl Error {
    fn negotiation_in_progress() -> &'static str {
        "Negotiation in progress"
    }

    fn cannot_remove_from_attacking_faction() -> &'static str {
        "Cannot remove stone from the attacking faction"
    }

    fn must_remove_when_attacking() -> &'static str {
        "Must remove at least one stone when attacking"
    }

    fn not_enough_stones_in_zone() -> &'static str {
        "Not enough stones in zone"
    }

    fn not_your_turn() -> &'static str {
        "Not your turn"
    }

    fn cannot_march_from_to() -> &'static str {
        "Cannot march between non-adjacent zones"
    }

    fn bad_player_count() -> &'static str {
        "The game supports between 2 and 5 players"
    }
}

#[derive(Debug)]
pub enum Action {
    StartNegotiation(Player),
    EndNegotiation(Player, Faction),
    March(Player, Faction, Zone, Zone, u8),
    Battle(Player, Zone, u8, u8, u8),
    Recruit(Player, Faction, Zone),
}

impl Action {
    fn player(&self) -> Player {
        match self {
            Action::StartNegotiation(player)
            | Action::EndNegotiation(player, _)
            | Action::March(player, _, _, _, _)
            | Action::Battle(player, _, _, _, _)
            | Action::Recruit(player, _, _) => *player,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Sequence, Debug)]
pub enum Zone {
    Red,   // Rogues' Zone
    Blue,  // Bullies' Zone
    Green, // Goons' Zone
    Orange,
    Yellow,
    Cyan,
    Magenta,
    Purple,
    White,
    Black,
    Gray,
}

#[derive(Clone, Copy, PartialEq, Eq, Sequence, Debug)]
pub enum Faction {
    Rogues,
    Bullies,
    Goons,
}

#[derive(Clone, Copy, PartialEq, Eq, Sequence, Default, Debug)]
pub enum Player {
    #[default]
    Alpha,
    Beta,
    Gamma,
    Delta,
    Epsilon,
}

/// Starts a new game with the given number of players.
/// Returns an error if the number of players is not between 2 and 5.
pub fn start_game(num_players: u8) -> Result<Board, &'static str> {
    Board::build(num_players)
}

pub fn take_turn(board: Board, action: Action) -> (Board, Result<Option<Option<Player>>, Error>) {
    board.process_action(action)
}