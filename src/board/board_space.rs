use std::cmp::Ordering;

use crate::{Error, Faction};

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoardSpace {
    rogues: u8,
    bullies: u8,
    goons: u8,
}

impl BoardSpace {
    pub fn home_base(faction: Faction) -> Self {
        let mut space = BoardSpace::default();
        *space.get_faction_mut(faction) = 2;
        space
    }

    pub fn subtract_faction(&mut self, faction: Faction, amount: u8) -> Result<(), &'static str> {
        let faction = self.get_faction_mut(faction);
        match faction.checked_sub(amount) {
            Some(diff) => {
                *faction = diff;
                Ok(())
            }
            None => Err(Error::not_enough_stones_in_zone()),
        }
    }

    pub fn add_faction(&mut self, faction: Faction, amount: u8) {
        *self.get_faction_mut(faction) += amount;
    }

    pub fn check_faction(self, faction: Faction, amount: u8) -> Result<(), &'static str> {
        if self.get_faction(faction) < amount {
            return Err(Error::not_enough_stones_in_zone());
        }
        Ok(())
    }

    pub fn winner(self, swords: Self, flags: Self) -> Option<Faction> {
        let mut flag_sort : Vec<Faction> = enum_iterator::all::<Faction>().collect();
        flag_sort.sort_unstable_by_key(|f| flags.get_faction(*f));
        let mut sword_sort : Vec<Faction> = enum_iterator::all::<Faction>().collect();
        sword_sort.sort_unstable_by_key(|f| swords.get_faction(*f));

        if self.rogues == self.bullies && self.rogues == self.goons {
            if swords.get_faction(sword_sort[0]) == swords.get_faction(sword_sort[1]) {
                if flags.get_faction(flag_sort[0]) == flags.get_faction(flag_sort[1]) {
                    return None;
                }
                return Some(flag_sort[0]);
            }
            return Some(sword_sort[0]);
        }

        if self.rogues > self.bullies && self.rogues > self.goons {
            return Some(Faction::Rogues);
        }
        if self.bullies > self.goons && self.bullies > self.rogues {
            return Some(Faction::Bullies);
        }
        if self.goons > self.rogues && self.goons > self.bullies {
            return Some(Faction::Goons);
        }

        let tie_breaker = |fac1, fac2| match swords.get_faction(fac1).cmp(&swords.get_faction(fac2))
        {
            Ordering::Greater => Some(fac1),
            Ordering::Less => Some(fac2),
            Ordering::Equal => match flags.get_faction(fac1).cmp(&flags.get_faction(fac2)) {
                Ordering::Greater => Some(fac1),
                Ordering::Less => Some(fac2),
                Ordering::Equal => None,
            },
        };

        if self.bullies == self.rogues {
            return tie_breaker(Faction::Bullies, Faction::Rogues);
        }

        if self.rogues == self.goons {
            return tie_breaker(Faction::Goons, Faction::Rogues);
        }

        if self.goons == self.bullies {
            return tie_breaker(Faction::Goons, Faction::Bullies);
        }

        None
    }

    pub fn loser(self) -> Option<Faction> {
        let inverse = BoardSpace {
            rogues: 255 - self.rogues,
            bullies: 255 - self.bullies,
            goons: 255 - self.goons,
        };
        inverse.winner(BoardSpace::default(), BoardSpace::default())
    }

    fn get_faction_mut(&mut self, faction: Faction) -> &mut u8 {
        match faction {
            Faction::Rogues => &mut self.rogues,
            Faction::Bullies => &mut self.bullies,
            Faction::Goons => &mut self.goons,
        }
    }

    fn get_faction(self, faction: Faction) -> u8 {
        match faction {
            Faction::Rogues => self.rogues,
            Faction::Bullies => self.bullies,
            Faction::Goons => self.goons,
        }
    }

    pub fn winning_sort(
        a: BoardSpace,
        b: BoardSpace,
        winning_faction: Faction,
        losing_faction: Option<Faction>,
    ) -> Ordering {
        let tiebreaker = if let Some(losing_faction) = losing_faction {
            a.get_faction(losing_faction)
                .cmp(&b.get_faction(losing_faction))
                .reverse()
        } else {
            Ordering::Equal
        };
        a.get_faction(winning_faction)
            .cmp(&b.get_faction(winning_faction))
            .then(tiebreaker)
    }
}
