use std::cmp::Ordering;

use crate::{Crew, Error};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub(super) struct BoardSpace {
    rogues: u8,
    bullies: u8,
    goons: u8,
}

impl BoardSpace {
    pub(super) const EMPTY: Self = Self{
        rogues: 0,
        bullies: 0,
        goons: 0,
    };

    pub(super) const fn home_base(crew: Crew) -> Self {
        match crew {
            Crew::Rogues => Self {
                rogues: 2,
                ..Self::EMPTY
            },
            Crew::Bullies => Self {
                bullies: 2,
                ..Self::EMPTY
            },
            Crew::Goons => Self {
                goons: 2,
                ..Self::EMPTY
            },
        }
    }

    pub(super) fn subtract_crew(&mut self, crew: Crew, amount: u8) -> Result<(), &'static str> {
        let crew = self.get_crew_mut(crew);
        crew.checked_sub(amount)
            .map_or(Err(Error::NOT_ENOUGH_STONES_IN_ZONE), |diff| {
                *crew = diff;
                Ok(())
            })
    }

    pub(super) fn add_crew(&mut self, crew: Crew, amount: u8) {
        *self.get_crew_mut(crew) += amount;
    }

    pub(super) const fn check_crew(self, crew: Crew, amount: u8) -> Result<(), &'static str> {
        if self.get_crew(crew) < amount {
            return Err(Error::NOT_ENOUGH_STONES_IN_ZONE);
        }
        Ok(())
    }

    pub(super) fn winner(self, swords: Self, flags: Self) -> Option<Crew> {
        let mut flag_sort: Vec<Crew> = enum_iterator::all::<Crew>().collect();
        flag_sort.sort_unstable_by_key(|f| flags.get_crew(*f));
        let mut sword_sort: Vec<Crew> = enum_iterator::all::<Crew>().collect();
        sword_sort.sort_unstable_by_key(|f| swords.get_crew(*f));

        if self.rogues == self.bullies && self.rogues == self.goons {
            if swords.get_crew(sword_sort[0]) == swords.get_crew(sword_sort[1]) {
                if flags.get_crew(flag_sort[0]) == flags.get_crew(flag_sort[1]) {
                    return None;
                }
                return Some(flag_sort[0]);
            }
            return Some(sword_sort[0]);
        }

        if self.rogues > self.bullies && self.rogues > self.goons {
            return Some(Crew::Rogues);
        }
        if self.bullies > self.goons && self.bullies > self.rogues {
            return Some(Crew::Bullies);
        }
        if self.goons > self.rogues && self.goons > self.bullies {
            return Some(Crew::Goons);
        }

        let tie_breaker = |fac1, fac2| match swords.get_crew(fac1).cmp(&swords.get_crew(fac2)) {
            Ordering::Greater => Some(fac1),
            Ordering::Less => Some(fac2),
            Ordering::Equal => match flags.get_crew(fac1).cmp(&flags.get_crew(fac2)) {
                Ordering::Greater => Some(fac1),
                Ordering::Less => Some(fac2),
                Ordering::Equal => None,
            },
        };

        if self.bullies == self.rogues {
            return tie_breaker(Crew::Bullies, Crew::Rogues);
        }

        if self.rogues == self.goons {
            return tie_breaker(Crew::Goons, Crew::Rogues);
        }

        if self.goons == self.bullies {
            return tie_breaker(Crew::Goons, Crew::Bullies);
        }

        None
    }

    pub(super) fn loser(self) -> Option<Crew> {
        let inverse = Self {
            rogues: 255 - self.rogues,
            bullies: 255 - self.bullies,
            goons: 255 - self.goons,
        };
        inverse.winner(Self::default(), Self::default())
    }

    fn get_crew_mut(&mut self, crew: Crew) -> &mut u8 {
        match crew {
            Crew::Rogues => &mut self.rogues,
            Crew::Bullies => &mut self.bullies,
            Crew::Goons => &mut self.goons,
        }
    }

    const fn get_crew(self, crew: Crew) -> u8 {
        match crew {
            Crew::Rogues => self.rogues,
            Crew::Bullies => self.bullies,
            Crew::Goons => self.goons,
        }
    }

    pub(super) fn winning_sort(
        a: Self,
        b: Self,
        winning_crew: Crew,
        losing_crew: Option<Crew>,
    ) -> Ordering {
        let tiebreaker = losing_crew
            .map_or(Ordering::Equal, |losing_crew| a.get_crew(losing_crew)
                .cmp(&b.get_crew(losing_crew))
                .reverse()
            );
        a.get_crew(winning_crew)
            .cmp(&b.get_crew(winning_crew))
            .then(tiebreaker)
    }
}
