use crate::Crew;
use rand::Rng;

#[derive(Clone, Copy, Default)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub(super) struct Bag {
    rogues: u8,
    goons: u8,
    bullies: u8,
}

impl Bag {
    pub(super) const EMPTY: Self = Self {
        rogues: 0,
        goons: 0,
        bullies: 0,
    };

    pub(super) fn from_slice(slice: &[Crew]) -> Self {
        let mut bag = Self::default();
        for crew in slice {
            match crew {
                Crew::Rogues => bag.rogues += 1,
                Crew::Goons => bag.goons += 1,
                Crew::Bullies => bag.bullies += 1,
            }
        }
        bag
    }

    pub(super) fn draw(&mut self) -> Crew {
        let total = self.rogues + self.goons + self.bullies;
        let mut rng = rand::thread_rng();
        let mut roll = rng.gen_range(0..total);
        if roll < self.rogues {
            self.rogues -= 1;
            Crew::Rogues
        } else {
            roll -= self.rogues;
            if roll < self.goons {
                self.goons -= 1;
                Crew::Goons
            } else {
                self.bullies -= 1;
                Crew::Bullies
            }
        }
    }

    pub(super) fn replace(&mut self, crew: Crew) {
        match crew {
            Crew::Rogues => self.rogues += 1,
            Crew::Goons => self.goons += 1,
            Crew::Bullies => self.bullies += 1,
        }
    }
}
