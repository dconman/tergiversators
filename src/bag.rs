use crate::Crew;
use rand::Rng;

#[derive(Clone, Copy, Default)]
pub(crate) struct Bag {
    rogues: u8,
    goons: u8,
    bullies: u8,
}

impl Bag {
    pub(crate) fn from_slice(slice: &[Crew]) -> Self {
        let mut bag = Bag::default();
        for crew in slice {
            match crew {
                Crew::Rogues => bag.rogues += 1,
                Crew::Goons => bag.goons += 1,
                Crew::Bullies => bag.bullies += 1,
            }
        }
        bag
    }
    pub(crate) fn draw(&mut self) -> Crew {
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

    pub(crate) fn replace(&mut self, crew: Crew) {
        match crew {
            Crew::Rogues => self.rogues += 1,
            Crew::Goons => self.goons += 1,
            Crew::Bullies => self.bullies += 1,
        }
    }
}
