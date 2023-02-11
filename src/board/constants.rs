use crate::{Crew, Zone};

// 57 start in the bag, 6 start on the board
#[rustfmt::skip]
pub(super) const DEFAULT_BAG: [Crew; 57] = [
    Crew::Bullies, Crew::Bullies, Crew::Bullies, Crew::Bullies, Crew::Bullies, Crew::Bullies, Crew::Bullies, Crew::Bullies, Crew::Bullies, Crew::Bullies, Crew::Bullies, Crew::Bullies, Crew::Bullies, Crew::Bullies, Crew::Bullies, Crew::Bullies, Crew::Bullies, Crew::Bullies, Crew::Bullies,
    Crew::Goons, Crew::Goons, Crew::Goons, Crew::Goons, Crew::Goons, Crew::Goons, Crew::Goons, Crew::Goons, Crew::Goons, Crew::Goons, Crew::Goons, Crew::Goons, Crew::Goons, Crew::Goons, Crew::Goons, Crew::Goons, Crew::Goons, Crew::Goons, Crew::Goons,
    Crew::Rogues, Crew::Rogues, Crew::Rogues, Crew::Rogues, Crew::Rogues, Crew::Rogues, Crew::Rogues, Crew::Rogues, Crew::Rogues, Crew::Rogues, Crew::Rogues, Crew::Rogues, Crew::Rogues, Crew::Rogues, Crew::Rogues, Crew::Rogues, Crew::Rogues, Crew::Rogues, Crew::Rogues,
];

#[rustfmt::skip]
pub(super) const ADJACENCIES: [(Zone, Zone); 34] = [
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

pub(super) const ZONES_TO_FILL: usize = 8;