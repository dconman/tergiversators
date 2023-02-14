use crate::Error;
use crate::{Action, Crew, Player, TurnResult, Winner, Zone};
use bag::Bag;
use board_space::BoardSpace;
use rand::seq::SliceRandom;

mod bag;
mod board_space;
mod constants;

#[allow(clippy::wildcard_imports)]
use constants::*;

/// The board is the game state. It tracks everything about the game.
#[derive(Clone, Copy)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Board {
    bag: Bag,

    red: BoardSpace,
    green: BoardSpace,
    blue: BoardSpace,
    orange: BoardSpace,
    yellow: BoardSpace,
    cyan: BoardSpace,
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
    current_negotiation: bool,
    consecutive_negotiations: u8,
}

impl Board {
    const EMPTY: Self = Self{
        red: BoardSpace::home_base(Crew::Rogues),
        green: BoardSpace::home_base(Crew::Goons),
        blue: BoardSpace::home_base(Crew::Bullies),
        orange: BoardSpace::EMPTY,
        yellow: BoardSpace::EMPTY,
        cyan: BoardSpace::EMPTY,
        magenta: BoardSpace::EMPTY,
        purple: BoardSpace::EMPTY,
        white: BoardSpace::EMPTY,
        black: BoardSpace::EMPTY,
        gray: BoardSpace::EMPTY,
        alpha: BoardSpace::EMPTY,
        beta: BoardSpace::EMPTY,
        gamma: BoardSpace::EMPTY,
        delta: BoardSpace::EMPTY,
        epsilon: BoardSpace::EMPTY,
        swords: BoardSpace::EMPTY,
        flags: BoardSpace::EMPTY,
        bag: Bag::EMPTY,
        next_player: Player::Alpha,
        current_negotiation: false,
        consecutive_negotiations: 0,
        num_players: 2,
    };

    pub(crate) fn build(num_players: u8) -> Result<Self, &'static str> {
        if !(2..=5).contains(&num_players) {
            return Err(Error::BAD_PLAYER_COUNT);
        }
        let mut board = Self {
            num_players,
            ..Self::EMPTY
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

    const fn get_space(&self, zone: Zone) -> &BoardSpace {
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

    const fn get_hand(&self, player: Player) -> &BoardSpace {
        match player {
            Player::Alpha => &self.alpha,
            Player::Beta => &self.beta,
            Player::Gamma => &self.gamma,
            Player::Delta => &self.delta,
            Player::Epsilon => &self.epsilon,
        }
    }

    fn setup(&mut self, num_players: usize) {
        let mut peices = [Crew::Rogues; 57];
        peices.copy_from_slice(&DEFAULT_BAG);
        peices.shuffle(&mut rand::thread_rng());

        for (zone, crews) in enum_iterator::all::<Zone>()
            .skip(3)
            .zip(peices.chunks_exact(2))
        {
            self.get_space_mut(zone).add_crew(crews[0], 1);
            self.get_space_mut(zone).add_crew(crews[1], 1);
        }

        for (player, crews) in enum_iterator::all::<Player>()
            .take(num_players)
            .zip(peices[ZONES_TO_FILL * 2..].chunks_exact(8))
        {
            let hand = self.get_hand_mut(player);
            for &f in crews {
                hand.add_crew(f, 1);
            }
        }

        self.bag = Bag::from_slice(&peices[(ZONES_TO_FILL * 2 + num_players * 8)..]);
    }

    fn play_crew(&mut self, player: Player, crew: Crew) -> Result<(), &'static str> {
        self.get_hand_mut(player).subtract_crew(crew, 1)
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
        crew: Crew,
        zone: Zone,
        rogues: u8,
        bullies: u8,
        goons: u8,
    ) -> Result<(), &'static str> {
        if match crew {
            Crew::Rogues => rogues,
            Crew::Bullies => bullies,
            Crew::Goons => goons,
        } > 0
        {
            return Err(Error::CANNOT_REMOVE_FROM_ATTACKING_FACTION);
        };
        let removal = rogues + bullies + goons;
        if removal == 0 {
            return Err(Error::MUST_REMOVE_WHEN_ATTACKING);
        }

        self.play_crew(player, crew)?;

        self.get_space(zone).check_crew(crew, removal)?;

        self.swords.add_crew(crew, 1);

        for (crew, &amount) in enum_iterator::all::<Crew>().zip([rogues, bullies, goons].iter()) {
            self.get_space_mut(zone).subtract_crew(crew, amount)?;
            self.bag.replace(crew);
        }

        Ok(())
    }

    fn march(
        &mut self,
        player: Player,
        crew: Crew,
        from: Zone,
        to: Zone,
        amount: u8,
    ) -> Result<(), &'static str> {
        if !ADJACENCIES.contains(&(from, to)) {
            return Err(Error::CANNOT_MARCH_FROM_TO);
        }
        self.play_crew(player, crew)?;
        self.get_space_mut(from).subtract_crew(crew, amount)?;
        self.flags.add_crew(crew, 1);
        self.get_space_mut(to).add_crew(crew, amount);
        Ok(())
    }

    // This will never return an error, but the signature should match the other methods
    #[allow(clippy::unnecessary_wraps)]
    fn start_negotiation(&mut self, player: Player) -> Result<(), &'static str> {
        let crew = self.bag.draw();
        self.get_hand_mut(player).add_crew(crew, 1);
        self.current_negotiation = true;
        Ok(())
    }

    fn end_negotiation(&mut self, player: Player, crew: Crew) -> Result<(), &'static str> {
        self.play_crew(player, crew)?;
        self.bag.replace(crew);
        self.current_negotiation = false;
        self.consecutive_negotiations += 1;
        Ok(())
    }

    fn recruit(&mut self, player: Player, crew: Crew, zone: Zone) -> Result<(), &'static str> {
        self.play_crew(player, crew)?;
        self.get_space_mut(zone).add_crew(crew, 1);
        Ok(())
    }

    fn score(&self) -> Option<Player> {
        let mut scores = BoardSpace::default();
        let num_players = self.num_players.into();

        for zone in enum_iterator::all::<Zone>() {
            if let Some(crew) = self.get_space(zone).controlling_crew(self.swords, self.flags) {
                scores.add_crew(crew, 1);
            }
        }

        let winning_crew = scores.controlling_crew(self.swords, self.flags)?;
        let losing_crew = scores.loser();

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
            BoardSpace::winning_sort(*a_hand, *b_hand, winning_crew, losing_crew).then_with(|| {
                play_order
                    .iter()
                    .position(|&p| p == a)
                    .cmp(&play_order.iter().position(|&p| p == b))
            })
        });

        Some(players[0])
    }

    pub(crate) fn process_action(self, action: Action) -> TurnResult {
        if self.current_negotiation && !matches!(action, Action::EndNegotiation(_)) {
            return TurnResult(
                self,
                Err(Error {
                    reason: Error::NEGOTIATION_IN_PROGRESS,
                    action,
                }),
            );
        }

        let mut next = self;
        let player = self.next_player;

        if !matches!(action, Action::StartNegotiation) {
            next.advance_turn();
        }

        let res = match action {
            Action::EndNegotiation(crew) => next.end_negotiation(player, crew),
            Action::Battle(crew, zone, red, blue, green) => {
                next.battle(player, crew, zone, red, blue, green)
            }
            Action::March(crew, from, to, amount) => next.march(player, crew, from, to, amount),
            Action::StartNegotiation => next.start_negotiation(player),
            Action::Recruit(crew, zone) => next.recruit(player, crew, zone),
        }
        .map_err(|reason| Error { action, reason })
        .map(|_| {
            if next.consecutive_negotiations >= next.num_players {
                Some(next.score().map_or(Winner::Draw, Winner::Player))
            } else {
                None
            }
        });

        TurnResult(next, res)
    }
}
