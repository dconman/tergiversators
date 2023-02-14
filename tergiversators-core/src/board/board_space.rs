use std::cmp::Ordering;

use crate::{Crew, Error};

/// A space on the board, including player hands.
#[derive(Default, Clone, Copy)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub(super) struct BoardSpace {
    rogues: u8,
    bullies: u8,
    goons: u8,
}

impl BoardSpace {
    /// Returns the number of crew members of the given type in this space.
    const fn get_crew(self, crew: Crew) -> u8 {
        match crew {
            Crew::Rogues => self.rogues,
            Crew::Bullies => self.bullies,
            Crew::Goons => self.goons,
        }
    }

    pub(super) const EMPTY: Self = Self {
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

    pub(super) fn controlling_crew(self, swords: Self, flags: Self) -> Option<Crew> {
        let flag_sort = flags.get_sorted_crews();
        let sword_sort = swords.get_sorted_crews();

        if self.rogues == self.goons && self.rogues == self.bullies {
            if swords.get_crew(sword_sort[0]) == swords.get_crew(sword_sort[1]) {
                if flags.get_crew(flag_sort[0]) == flags.get_crew(flag_sort[1]) {
                    return None;
                }
                return Some(flag_sort[0]);
            }
            return Some(sword_sort[0]);
        }

        let self_sort = self.get_sorted_crews();

        if self.get_crew(self_sort[0]) != self.get_crew(self_sort[1]) {
            return Some(self_sort[0]);
        }

        match swords
            .get_crew(self_sort[0])
            .cmp(&swords.get_crew(self_sort[1]))
        {
            Ordering::Greater => Some(self_sort[0]),
            Ordering::Less => Some(self_sort[1]),
            Ordering::Equal => match flags
                .get_crew(self_sort[0])
                .cmp(&flags.get_crew(self_sort[1]))
            {
                Ordering::Greater => Some(self_sort[0]),
                Ordering::Less => Some(self_sort[1]),
                Ordering::Equal => None,
            },
        }
    }

    pub(super) fn loser(self) -> Option<Crew> {
        let inverse = Self {
            rogues: 255 - self.rogues,
            bullies: 255 - self.bullies,
            goons: 255 - self.goons,
        };
        inverse.controlling_crew(Self::default(), Self::default())
    }

    fn get_crew_mut(&mut self, crew: Crew) -> &mut u8 {
        match crew {
            Crew::Rogues => &mut self.rogues,
            Crew::Bullies => &mut self.bullies,
            Crew::Goons => &mut self.goons,
        }
    }

    fn get_sorted_crews(self) -> [Crew; 3] {
        let mut crews = [Crew::Rogues, Crew::Bullies, Crew::Goons];
        crews.sort_unstable_by_key(|c| self.get_crew(*c));
        crews.reverse();
        crews
    }

    pub(super) fn winning_sort(
        a: Self,
        b: Self,
        winning_crew: Crew,
        losing_crew: Option<Crew>,
    ) -> Ordering {
        let tiebreaker = losing_crew.map_or(Ordering::Equal, |losing_crew| {
            a.get_crew(losing_crew)
                .cmp(&b.get_crew(losing_crew))
                .reverse()
        });
        a.get_crew(winning_crew)
            .cmp(&b.get_crew(winning_crew))
            .then(tiebreaker)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn subtract_crew_removes_crew() {
        let mut space = BoardSpace::home_base(Crew::Rogues);
        space.subtract_crew(Crew::Rogues, 1).unwrap();
        assert_eq!(
            space,
            BoardSpace {
                rogues: 1,
                ..BoardSpace::EMPTY
            }
        );
    }

    #[test]
    fn subtract_crew_fails_if_not_enough_crew() {
        let mut space = BoardSpace::home_base(Crew::Rogues);
        assert_eq!(
            space.subtract_crew(Crew::Rogues, 3),
            Err(Error::NOT_ENOUGH_STONES_IN_ZONE)
        );
    }

    #[test]
    fn add_crew_adds_crew() {
        let mut space = BoardSpace::home_base(Crew::Rogues);
        space.add_crew(Crew::Rogues, 1);
        assert_eq!(
            space,
            BoardSpace {
                rogues: 3,
                ..BoardSpace::EMPTY
            }
        );
    }

    #[test]
    fn check_crew_passes_if_enough_crew() {
        let space = BoardSpace::home_base(Crew::Rogues);
        assert_eq!(space.check_crew(Crew::Rogues, 2), Ok(()));
    }

    #[test]
    fn check_crew_fails_if_not_enough_crew() {
        let space = BoardSpace::home_base(Crew::Rogues);
        assert_eq!(
            space.check_crew(Crew::Rogues, 3),
            Err(Error::NOT_ENOUGH_STONES_IN_ZONE)
        );
    }

    mod controlling_crew {
        use super::*;

        mod when_not_tied {
            use super::*;

            #[test]
            fn returns_winning_faction() {
                let space = BoardSpace {
                    rogues: 2,
                    bullies: 1,
                    goons: 1,
                };
                assert_eq!(
                    space.controlling_crew(BoardSpace::EMPTY, BoardSpace::EMPTY),
                    Some(Crew::Rogues)
                );
            }
        }

        mod when_three_way_tied {
            use super::*;

            mod when_swords_has_a_winner {
                use super::*;

                #[test]
                fn returns_sword_winner() {
                    let space = BoardSpace {
                        rogues: 1,
                        bullies: 1,
                        goons: 1,
                    };
                    let swords = BoardSpace {
                        rogues: 1,
                        bullies: 0,
                        goons: 3,
                    };
                    assert_eq!(
                        space.controlling_crew(swords, BoardSpace::EMPTY),
                        Some(Crew::Goons)
                    );
                }
            }

            mod when_swords_tied {
                use super::*;

                mod when_flags_not_ties {
                    use super::*;

                    #[test]
                    fn returns_flag_winner() {
                        let space = BoardSpace {
                            rogues: 1,
                            bullies: 1,
                            goons: 1,
                        };
                        let swords = BoardSpace {
                            rogues: 1,
                            bullies: 0,
                            goons: 1,
                        };
                        let flags = BoardSpace {
                            rogues: 1,
                            bullies: 3,
                            goons: 1,
                        };
                        assert_eq!(space.controlling_crew(swords, flags), Some(Crew::Bullies));
                    }
                }

                mod when_flags_tied {
                    use super::*;

                    #[test]
                    fn returns_none() {
                        let space = BoardSpace {
                            rogues: 1,
                            bullies: 1,
                            goons: 1,
                        };
                        assert_eq!(space.controlling_crew(BoardSpace::EMPTY, BoardSpace::EMPTY), None);
                    }
                }
            }
        }
    }

    mod winning_sort {
        use super::*;

        mod when_not_tied {
            use super::*;

            #[test]
            fn returns_greater_if_a_has_more_winning_crew() {
                let a = BoardSpace {
                    rogues: 2,
                    bullies: 1,
                    goons: 1,
                };
                let b = BoardSpace {
                    rogues: 1,
                    bullies: 2,
                    goons: 1,
                };
                assert_eq!(
                    BoardSpace::winning_sort(a, b, Crew::Rogues, None),
                    Ordering::Greater
                );
            }

            #[test]
            fn returns_less_if_b_has_more_winning_crew() {
                let a = BoardSpace {
                    rogues: 1,
                    bullies: 2,
                    goons: 1,
                };
                let b = BoardSpace {
                    rogues: 2,
                    bullies: 1,
                    goons: 1,
                };
                assert_eq!(
                    BoardSpace::winning_sort(a, b, Crew::Rogues, None),
                    Ordering::Less
                );
            }
        }

        mod when_tied_on_winning_crew {
            use super::*;

            #[test]
            fn returns_less_if_a_has_more_losing_crew() {
                let a = BoardSpace {
                    rogues: 1,
                    bullies: 2,
                    goons: 1,
                };
                let b = BoardSpace {
                    rogues: 1,
                    bullies: 1,
                    goons: 2,
                };
                assert_eq!(
                    BoardSpace::winning_sort(a, b, Crew::Rogues, Some(Crew::Bullies)),
                    Ordering::Less
                );
            }

            #[test]
            fn returns_greater_if_b_has_more_losing_crew() {
                let a = BoardSpace {
                    rogues: 1,
                    bullies: 1,
                    goons: 2,
                };
                let b = BoardSpace {
                    rogues: 1,
                    bullies: 2,
                    goons: 1,
                };
                assert_eq!(
                    BoardSpace::winning_sort(a, b, Crew::Rogues, Some(Crew::Bullies)),
                    Ordering::Greater
                );
            }

            #[test]
            fn returns_equal_if_both_have_same_losing_crew() {
                let a = BoardSpace {
                    rogues: 1,
                    bullies: 1,
                    goons: 2,
                };
                let b = BoardSpace {
                    rogues: 1,
                    bullies: 1,
                    goons: 2,
                };
                assert_eq!(
                    BoardSpace::winning_sort(a, b, Crew::Rogues, Some(Crew::Bullies)),
                    Ordering::Equal
                );
            }

            #[test]
            fn returns_equal_if_no_losing_crew() {
                let a = BoardSpace {
                    rogues: 1,
                    bullies: 1,
                    goons: 2,
                };
                let b = BoardSpace {
                    rogues: 1,
                    bullies: 1,
                    goons: 2,
                };
                assert_eq!(
                    BoardSpace::winning_sort(a, b, Crew::Rogues, None),
                    Ordering::Equal
                );
            }
        }
    }
}
