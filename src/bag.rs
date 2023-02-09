
use rand::Rng;
use crate::Faction;


#[derive(Debug, Clone, Copy, Default)]
pub struct Bag {
    rogues: u8,
    goons: u8,
    bullies: u8,
}

impl Bag {
    pub fn from_slice(slice: &[Faction]) -> Self {
        let mut bag = Bag::default();
        for faction in slice {
            match faction {
                Faction::Rogues => bag.rogues += 1,
                Faction::Goons => bag.goons += 1,
                Faction::Bullies => bag.bullies += 1,
            }
        }
        bag
    }
    pub fn draw(&mut self) -> Faction {
        let total = self.rogues + self.goons + self.bullies;
        let mut rng = rand::thread_rng();
        let mut roll = rng.gen_range(0..total);
        if roll < self.rogues {
            self.rogues -= 1;
            Faction::Rogues
        } else {
            roll -= self.rogues;
            if roll < self.goons {
                self.goons -= 1;
                Faction::Goons
            } else {
                self.bullies -= 1;
                Faction::Bullies
            }
        }
    }

    pub fn replace(&mut self, faction: Faction) {
        match faction {
            Faction::Rogues => self.rogues += 1,
            Faction::Goons => self.goons += 1,
            Faction::Bullies => self.bullies += 1,
        }
    }
}

