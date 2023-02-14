#![cfg(debug_assertions)]

use tergiversators::*;

fn translate_zone(input: &str) -> Option<Zone> {
    match input {
        "red" => Some(Zone::Red),
        "blue" => Some(Zone::Blue),
        "green" => Some(Zone::Green),
        "orange" => Some(Zone::Orange),
        "yellow" => Some(Zone::Yellow),
        "cyan" => Some(Zone::Cyan),
        "magenta" => Some(Zone::Magenta),
        "purple" => Some(Zone::Purple),
        "white" => Some(Zone::White),
        "black" => Some(Zone::Black),
        "gray" => Some(Zone::Gray),
        _ => None,
    }
}

fn translate_crew(input: &str) -> Option<Crew> {
    match input {
        "bullies" => Some(Crew::Bullies),
        "goons" => Some(Crew::Goons),
        "rogues" => Some(Crew::Rogues),
        _ => None,
    }
}

fn get_zone() -> Zone {
    let mut line = String::new();
    loop {
        std::io::stdin().read_line(&mut line).unwrap();
        if let Some(zone) = translate_zone(line.trim()) {
            return zone;
        }
        println!("Invalid zone");
        line.clear();
    }
}

fn get_number() -> u8 {
    let mut line = String::new();
    loop {
        std::io::stdin().read_line(&mut line).unwrap();
        if let Ok(num) = line.trim().parse::<u8>() {
            return num;
        }
        println!("Invalid number");
        line.clear();
    }
}

fn get_crew() -> Crew {
    let mut line = String::new();
    loop {
        std::io::stdin().read_line(&mut line).unwrap();
        if let Some(crew) = translate_crew(line.trim()) {
            return crew;
        }
        println!("Invalid crew");
        line.clear();
    }
}

fn recruit() -> Action {
    println!("What crew?");
    let crew = get_crew();
    println!("Where?");
    let zone = get_zone();
    Action::Recruit(crew, zone)
}

fn march() -> Action {
    println!("What crew?");
    let crew = get_crew();
    println!("From?");
    let from = get_zone();
    println!("To?");
    let to = get_zone();
    println!("How Many?");
    let num = get_number();
    Action::March(crew, from, to, num)
}

fn end_negotiation() -> Action {
    println!("What crew?");
    let crew = get_crew();
    Action::EndNegotiation(crew)
}

fn battle() -> Action {
    println!("Who's attacking?");
    let crew = get_crew();
    println!("Where?");
    let zone = get_zone();
    let mut rogues = 0;
    let mut goons = 0;
    let mut bullies = 0;
    if crew != Crew::Rogues {
        println!("How many rogues?");
        rogues = get_number();
    }
    if crew != Crew::Goons {
        println!("How many goons?");
        goons = get_number();
    }
    if crew != Crew::Bullies {
        println!("How many bullies?");
        bullies = get_number();
    }
    Action::Battle(crew, zone, rogues, bullies, goons)
}

fn translate_action(input: &str) -> Option<Action> {
    match input {
        "recruit" => Some(recruit()),
        "march" => Some(march()),
        "negotiate" => Some(Action::StartNegotiation),
        "battle" => Some(battle()),
        _=> None,
    }
}

fn get_action() -> Action {
    let mut line = String::new();
    loop {
        std::io::stdin().read_line(&mut line).unwrap();
        if let Some(action) = translate_action(line.trim()) {
            return action;
        }
        println!("Invalid action");
        line.clear();
    }
}

fn get_winner(player: Winner) -> &'static str {
    match player {
        Winner::Draw => "Draw",
        Winner::Player(Player::Alpha) => "First Player",
        Winner::Player(Player::Beta) => "Second Player",
        Winner::Player(Player::Gamma) => "Third Player",
        Winner::Player(Player::Delta) => "Fourth Player",
        Winner::Player(Player::Epsilon) => "Fifth Player",
    }
}

fn main() {
    let mut line = String::new();
    println!("Number of players?");
    std::io::stdin().read_line(&mut line).unwrap();
    let num_players = line.trim().parse::<u8>().unwrap();
    let mut board = start_game(num_players).unwrap();
    let mut result: Result<Option<Winner>, Error> ;
    let mut negotiation = false;
    loop {
        println!("Board: {:?}", board);
        let action = if negotiation {
            end_negotiation()
        } else {
            println!("What action?");
            get_action()
        };
        negotiation = matches!(action, Action::StartNegotiation);
        TurnResult(board, result) = take_turn(board, action);
        match result {
            Ok(Some(winner)) => {
                println!("Winner: {:?}", get_winner(winner));
                break;
            },
            Err(Error{reason, ..}) => {
                println!("Error: {:?}", reason);
            },
            _ => (),
        }
    }



}