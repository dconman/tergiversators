//          _____
//         /     \
//        /       \_____
//        \             \
//         \   Green     \
//         /   Goons     /
//   _____/        _____/        _____         _____
//  /     \       /     \       /     \       /     \
// /       \_____/       \_____/       \_____/       \_____
// \             \             \             \             \
//  \   Yellow    \    Cyan     \   Magenta   \   Blue      \
//  /             /             /             /   Bullies   /
// /        _____/        _____/        _____/        _____/
// \       /     \       /     \       /     \       /     \
//  \_____/       \_____/ XXXXX \_____/       \_____/       \_____
//        \             \ XXXXXX      \             \             \
//         \   Orange    \ XXXXXXXXXXX \   Purple    \   White     \
//         /             / XXXXXXXXXXX /             /             /
//        /        _____/ XXXXXX _____/        _____/        _____/
//        \       /     \ XXXXX /     \       /     \       /
//         \_____/       \_____/       \_____/       \_____/
//               \             \             \             \
//                \    Red      \    Grey     \   Black     \
//                /   Rogues    /             /             /
//               /        _____/        _____/        _____/
//               \       /     \       /     \       /
//                \_____/       \_____/       \_____/

use crate::bag::Bag;
use crate::board::board_space::BoardSpace;
use crate::Error;
use crate::{Action, Faction, Player, Zone};
use rand::seq::SliceRandom;

mod board_space;

#[rustfmt::skip]
const ADJACENCIES: [(Zone, Zone); 34] = [
    (Zone::Green  , Zone::Cyan   ), (Zone::Green  , Zone::Yellow ),
    (Zone::Red    , Zone::Gray   ), (Zone::Red    , Zone::Orange ),
    (Zone::Black  , Zone::Gray   ), (Zone::Black  , Zone::Purple ), (Zone::Black  , Zone::White  ),
    (Zone::Blue   , Zone::Magenta), (Zone::Blue   , Zone::Purple ), (Zone::Blue   , Zone::White  ),
    (Zone::Gray   , Zone::Black  ), (Zone::Gray   , Zone::Purple ), (Zone::Gray   , Zone::Red    ),
    (Zone::Magenta, Zone::Blue   ), (Zone::Magenta, Zone::Cyan   ), (Zone::Magenta, Zone::Purple ),
    (Zone::Orange , Zone::Cyan   ), (Zone::Orange , Zone::Red    ), (Zone::Orange , Zone::Yellow ),
    (Zone::White  , Zone::Black  ), (Zone::White  , Zone::Blue   ), (Zone::White  , Zone::Purple ),
    (Zone::Yellow , Zone::Cyan   ), (Zone::Yellow , Zone::Green  ), (Zone::Yellow , Zone::Orange ),
    (Zone::Cyan   , Zone::Green  ), (Zone::Cyan   , Zone::Magenta), (Zone::Cyan   , Zone::Orange ), (Zone::Cyan   , Zone::Yellow ),
    (Zone::Purple , Zone::Black  ), (Zone::Purple , Zone::Blue   ), (Zone::Purple , Zone::Gray   ), (Zone::Purple , Zone::Magenta), (Zone::Purple , Zone::White  ),
];

// 57 start in the bag, 6 start on the board
#[rustfmt::skip]
const DEFAULT_BAG: [Faction; 57] = [
    Faction::Bullies, Faction::Bullies, Faction::Bullies, Faction::Bullies, Faction::Bullies, Faction::Bullies, Faction::Bullies, Faction::Bullies, Faction::Bullies, Faction::Bullies, Faction::Bullies, Faction::Bullies, Faction::Bullies, Faction::Bullies, Faction::Bullies, Faction::Bullies, Faction::Bullies, Faction::Bullies, Faction::Bullies,
    Faction::Goons, Faction::Goons, Faction::Goons, Faction::Goons, Faction::Goons, Faction::Goons, Faction::Goons, Faction::Goons, Faction::Goons, Faction::Goons, Faction::Goons, Faction::Goons, Faction::Goons, Faction::Goons, Faction::Goons, Faction::Goons, Faction::Goons, Faction::Goons, Faction::Goons,
    Faction::Rogues, Faction::Rogues, Faction::Rogues, Faction::Rogues, Faction::Rogues, Faction::Rogues, Faction::Rogues, Faction::Rogues, Faction::Rogues, Faction::Rogues, Faction::Rogues, Faction::Rogues, Faction::Rogues, Faction::Rogues, Faction::Rogues, Faction::Rogues, Faction::Rogues, Faction::Rogues, Faction::Rogues,
];

const ZONES_TO_FILL: usize = 8;

#[derive(Default, Clone, Copy)]
pub struct Board {
    bag: Bag,

    red: BoardSpace,
    orange: BoardSpace,
    yellow: BoardSpace,
    green: BoardSpace,
    cyan: BoardSpace,
    blue: BoardSpace,
    magenta: BoardSpace,
    purple: BoardSpace,
    white: BoardSpace,
    black: BoardSpace,
    gray: BoardSpace,

    alpha: BoardSpace,
    beta: BoardSpace,
    gamma: BoardSpace,
    delta: BoardSpace,
    epsilon: BoardSpace,

    swords: BoardSpace,
    flags: BoardSpace,

    num_players: u8,
    next_player: Player,
    negotiation: Option<Player>,
    consecutive_negotiations: u8,
}

impl Board {
    pub fn build(num_players: u8) -> Result<Self, &'static str> {
        if !(2..=5).contains(&num_players) {
            return Err(Error::bad_player_count());
        }
        let mut board = Board {
            red: BoardSpace::home_base(Faction::Rogues),
            green: BoardSpace::home_base(Faction::Goons),
            blue: BoardSpace::home_base(Faction::Bullies),
            num_players,
            ..Board::default()
        };
        board.setup(num_players.into());
        Ok(board)
    }

    fn get_space_mut(&mut self, zone: Zone) -> &mut BoardSpace {
        match zone {
            Zone::Red => &mut self.red,
            Zone::Orange => &mut self.orange,
            Zone::Yellow => &mut self.yellow,
            Zone::Green => &mut self.green,
            Zone::Cyan => &mut self.cyan,
            Zone::Blue => &mut self.blue,
            Zone::Magenta => &mut self.magenta,
            Zone::Purple => &mut self.purple,
            Zone::White => &mut self.white,
            Zone::Black => &mut self.black,
            Zone::Gray => &mut self.gray,
        }
    }

    fn get_space(&self, zone: Zone) -> &BoardSpace {
        match zone {
            Zone::Red => &self.red,
            Zone::Orange => &self.orange,
            Zone::Yellow => &self.yellow,
            Zone::Green => &self.green,
            Zone::Cyan => &self.cyan,
            Zone::Blue => &self.blue,
            Zone::Magenta => &self.magenta,
            Zone::Purple => &self.purple,
            Zone::White => &self.white,
            Zone::Black => &self.black,
            Zone::Gray => &self.gray,
        }
    }

    fn get_hand_mut(&mut self, player: Player) -> &mut BoardSpace {
        match player {
            Player::Alpha => &mut self.alpha,
            Player::Beta => &mut self.beta,
            Player::Gamma => &mut self.gamma,
            Player::Delta => &mut self.delta,
            Player::Epsilon => &mut self.epsilon,
        }
    }

    fn get_hand(&self, player: Player) -> &BoardSpace {
        match player {
            Player::Alpha => &self.alpha,
            Player::Beta => &self.beta,
            Player::Gamma => &self.gamma,
            Player::Delta => &self.delta,
            Player::Epsilon => &self.epsilon,
        }
    }

    fn setup(&mut self, num_players: usize) {
        let mut peices = [Faction::Rogues; 57];
        peices.copy_from_slice(&DEFAULT_BAG);
        peices.shuffle(&mut rand::thread_rng());

        for (zone, factions) in enum_iterator::all::<Zone>()
            .skip(3)
            .zip(peices.chunks_exact(2))
        {
            self.get_space_mut(zone).add_faction(factions[0], 1);
            self.get_space_mut(zone).add_faction(factions[1], 1);
        }

        for (player, factions) in enum_iterator::all::<Player>()
            .take(num_players)
            .zip(peices[ZONES_TO_FILL * 2..].chunks_exact(8))
        {
            let hand = self.get_hand_mut(player);
            for &f in factions {
                hand.add_faction(f, 1);
            }
        }

        self.bag = Bag::from_slice(&peices[(ZONES_TO_FILL * 2 + num_players * 8)..]);
    }

    fn play_faction(&mut self, player: Player, faction: Faction) -> Result<(), &'static str> {
        self.get_hand_mut(player).subtract_faction(faction, 1)
    }

    fn advance_turn(&mut self) {
        self.next_player = match self.next_player {
            Player::Alpha => Player::Beta,
            Player::Beta => Player::Gamma,
            Player::Gamma => {
                if self.num_players > 2 {
                    Player::Delta
                } else {
                    Player::Alpha
                }
            }
            Player::Delta => {
                if self.num_players > 3 {
                    Player::Epsilon
                } else {
                    Player::Alpha
                }
            }
            Player::Epsilon => Player::Alpha,
        };
    }

    fn battle(
        &mut self,
        player: Player,
        zone: Zone,
        rogues: u8,
        bullies: u8,
        goons: u8,
    ) -> Result<(), &'static str> {
        let (removal, attacking_faction) = match (rogues, goons, bullies) {
            (0, 0, 0) => return Err(Error::must_remove_when_attacking()),
            (0, x, y) => (x + y, Faction::Rogues),
            (x, 0, y) => (x + y, Faction::Bullies),
            (x, y, 0) => (x + y, Faction::Goons),
            _ => return Err(Error::cannot_remove_from_attacking_faction()),
        };

        self.play_faction(player, attacking_faction)?;

        self.get_space(zone)
            .check_faction(attacking_faction, removal)?;

        self.swords.add_faction(attacking_faction, 1);

        for (faction, &amount) in
            enum_iterator::all::<Faction>().zip([rogues, bullies, goons].iter())
        {
            self.get_space_mut(zone).subtract_faction(faction, amount)?;
            self.bag.replace(faction);
        }

        Ok(())
    }

    fn march(
        &mut self,
        player: Player,
        faction: Faction,
        from: Zone,
        to: Zone,
        amount: u8,
    ) -> Result<(), &'static str> {
        if !ADJACENCIES.contains(&(from, to)) {
            return Err(Error::cannot_march_from_to());
        }
        self.play_faction(player, faction)?;
        self.get_space_mut(from).subtract_faction(faction, amount)?;
        self.flags.add_faction(faction, 1);
        self.get_space_mut(to).add_faction(faction, amount);
        Ok(())
    }

    // This will never return an error, but the signature should match the other methods
    #[allow(clippy::unnecessary_wraps)]
    fn start_negotiation(&mut self, player: Player) -> Result<(), &'static str> {
        let faction = self.bag.draw();
        self.get_hand_mut(player).add_faction(faction, 1);
        self.negotiation = Some(player);
        Ok(())
    }

    fn end_negotiation(&mut self, player: Player, faction: Faction) -> Result<(), &'static str> {
        self.play_faction(player, faction)?;
        self.bag.replace(faction);
        self.negotiation = None;
        self.consecutive_negotiations += 1;
        Ok(())
    }

    fn recruit(
        &mut self,
        player: Player,
        faction: Faction,
        zone: Zone,
    ) -> Result<(), &'static str> {
        self.play_faction(player, faction)?;
        self.get_space_mut(zone).add_faction(faction, 1);
        Ok(())
    }

    fn score(&self) -> Option<Player> {
        let mut scores = BoardSpace::default();
        let num_players = self.num_players.into();

        for zone in enum_iterator::all::<Zone>() {
            if let Some(faction) = self.get_space(zone).winner(self.swords, self.flags) {
                scores.add_faction(faction, 1);
            }
        }

        let winning_faction = scores.winner(self.swords, self.flags)?;
        let losing_faction = scores.loser();

        let play_order: Vec<Player> = enum_iterator::all::<Player>()
            .take(num_players)
            .cycle()
            .skip_while(|&p| p != self.next_player)
            .take(num_players)
            .collect();

        let mut players: Vec<Player> = enum_iterator::all::<Player>().collect();
        players.sort_unstable_by(|&a, &b| {
            let a_hand = self.get_hand(a);
            let b_hand = self.get_hand(b);
            BoardSpace::winning_sort(*a_hand, *b_hand, winning_faction, losing_faction).then_with(
                || {
                    play_order
                        .iter()
                        .position(|&p| p == a)
                        .cmp(&play_order.iter().position(|&p| p == b))
                },
            )
        });

        Some(players[0])
    }

    pub fn process_action(self, action: Action) -> (Board, Result<Option<Option<Player>>, Error>) {
        if self.negotiation.is_some() && !matches!(action, Action::EndNegotiation(_, _)) {
            return (
                self,
                Err(Error {
                    reason: Error::negotiation_in_progress(),
                    action,
                }),
            );
        }

        if action.player() != self.next_player {
            return (
                self,
                Err(Error {
                    reason: Error::not_your_turn(),
                    action,
                }),
            );
        }

        let mut next = self;

        if !matches!(action, Action::StartNegotiation(_)) {
            next.advance_turn();
        }

        let res = match action {
            Action::EndNegotiation(player, faction) => next.end_negotiation(player, faction),
            Action::Battle(player, zone, red, blue, green) => {
                next.battle(player, zone, red, blue, green)
            }
            Action::March(player, faction, from, to, amount) => {
                next.march(player, faction, from, to, amount)
            }
            Action::StartNegotiation(player) => next.start_negotiation(player),
            Action::Recruit(player, faction, zone) => next.recruit(player, faction, zone),
        }
        .map_err(|reason| Error { action, reason })
        .map(|_| {
            if next.consecutive_negotiations >= next.num_players {
                Some(next.score())
            } else {
                None
            }
        });

        (next, res)
    }
}
