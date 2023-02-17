use tergiversators_core::*;

fn show_board(board: &Board) {
    println!(
        r#"
          _____
         /     \                       R  B  G
        /  {:2}   \_____            🚩: {:2} {:2} {:2}
        \             \           ⚔️ : {:2} {:2} {:2}
         \   Green  {:2} \
         /   Goons     /
   _____/        _____/        _____         _____
  /     \  {:2}   /     \       /     \       /     \
 /  {:2}   \_____/  {:2}   \_____/  {:2}   \_____/  {:2}   \_____
 \             \             \             \             \
  \   Yellow {:2} \    Cyan  {:2} \  Magenta {:2} \   Blue   {:2} \
  /             /             /             /   Bullies   /
 /        _____/        _____/        _____/        _____/
 \  {:2}   /     \  {:2}   /     \  {:2}   /     \  {:2}   /     \
  \_____/  {:2}   \_____/   R   \_____/  {:2}   \_____/  {:2}   \_____
        \             \    \        \             \             \
         \   Orange {:2} \   CREW      \   Purple {:2} \   White  {:2} \
         /             /   LEGEND--B /             /             /
        /        _____/    /   _____/        _____/        _____/
        \  {:2}   /     \   G   /     \  {:2}   /     \  {:2}   /
         \_____/  {:2}   \_____/  {:2}   \_____/  {:2}   \_____/
               \             \             \             \
       R  B  G  \    Red   {:2} \    Gray  {:2} \   Black  {:2} \
   🫱: {:2} {:2} {:2}  /   Rogues    /             /             /
               /        _____/        _____/        _____/
               \  {:2}   /     \  {:2}   /     \  {:2}   /
                \_____/       \_____/       \_____/
"#,
        board.get_space(Zone::Green).get_crew(Crew::Rogues),
        board.get_flags().get_crew(Crew::Rogues),
        board.get_flags().get_crew(Crew::Bullies),
        board.get_flags().get_crew(Crew::Goons),
        board.get_swords().get_crew(Crew::Rogues),
        board.get_swords().get_crew(Crew::Bullies),
        board.get_swords().get_crew(Crew::Goons),
        board.get_space(Zone::Green).get_crew(Crew::Bullies),
        board.get_space(Zone::Green).get_crew(Crew::Goons),
        board.get_space(Zone::Yellow).get_crew(Crew::Rogues),
        board.get_space(Zone::Cyan).get_crew(Crew::Rogues),
        board.get_space(Zone::Magenta).get_crew(Crew::Rogues),
        board.get_space(Zone::Blue).get_crew(Crew::Rogues),
        board.get_space(Zone::Yellow).get_crew(Crew::Bullies),
        board.get_space(Zone::Cyan).get_crew(Crew::Bullies),
        board.get_space(Zone::Magenta).get_crew(Crew::Bullies),
        board.get_space(Zone::Blue).get_crew(Crew::Bullies),
        board.get_space(Zone::Yellow).get_crew(Crew::Goons),
        board.get_space(Zone::Cyan).get_crew(Crew::Goons),
        board.get_space(Zone::Magenta).get_crew(Crew::Goons),
        board.get_space(Zone::Blue).get_crew(Crew::Goons),
        board.get_space(Zone::Orange).get_crew(Crew::Rogues),
        board.get_space(Zone::Purple).get_crew(Crew::Rogues),
        board.get_space(Zone::White).get_crew(Crew::Rogues),
        board.get_space(Zone::Orange).get_crew(Crew::Bullies),
        board.get_space(Zone::Purple).get_crew(Crew::Bullies),
        board.get_space(Zone::White).get_crew(Crew::Bullies),
        board.get_space(Zone::Orange).get_crew(Crew::Goons),
        board.get_space(Zone::Purple).get_crew(Crew::Goons),
        board.get_space(Zone::White).get_crew(Crew::Goons),
        board.get_space(Zone::Red).get_crew(Crew::Rogues),
        board.get_space(Zone::Gray).get_crew(Crew::Rogues),
        board.get_space(Zone::Black).get_crew(Crew::Rogues),
        board.get_space(Zone::Red).get_crew(Crew::Bullies),
        board.get_space(Zone::Gray).get_crew(Crew::Bullies),
        board.get_space(Zone::Black).get_crew(Crew::Bullies),
        board.get_current_hand().get_crew(Crew::Rogues),
        board.get_current_hand().get_crew(Crew::Bullies),
        board.get_current_hand().get_crew(Crew::Goons),
        board.get_space(Zone::Red).get_crew(Crew::Goons),
        board.get_space(Zone::Gray).get_crew(Crew::Goons),
        board.get_space(Zone::Black).get_crew(Crew::Goons),
    );
}

fn get_crew(prompt: &str) -> Crew {
    if !prompt.is_empty() {
        print!("{prompt} ");
    }
    loop {
        println!("Crew?");
        let mut crew = String::new();
        std::io::stdin().read_line(&mut crew).unwrap();
        match crew.trim() {
            "Rogues" => return Crew::Rogues,
            "Bullies" => return Crew::Bullies,
            "Goons" => return Crew::Goons,
            _ => println!("Invalid crew: {}", crew),
        }
    }
}

fn get_zone(prompt: &str) -> Zone {
    if !prompt.is_empty() {
        print!("{prompt} ");
    }
    loop {
        println!("Zone?");
        let mut zone = String::new();
        std::io::stdin().read_line(&mut zone).unwrap();
        match zone.trim() {
            "Green" => return Zone::Green,
            "Yellow" => return Zone::Yellow,
            "Cyan" => return Zone::Cyan,
            "Magenta" => return Zone::Magenta,
            "Blue" => return Zone::Blue,
            "Orange" => return Zone::Orange,
            "Purple" => return Zone::Purple,
            "White" => return Zone::White,
            "Red" => return Zone::Red,
            "Gray" => return Zone::Gray,
            "Black" => return Zone::Black,
            _ => println!("Invalid zone: {}", zone),
        }
    }
}

fn get_u8(prompt: &str) -> u8 {
    if !prompt.is_empty() {
        print!("{prompt} ");
    }
    loop {
        println!("Number?");
        let mut num = String::new();
        std::io::stdin().read_line(&mut num).unwrap();
        match num.trim().parse::<u8>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid number: {}", num),
        }
    }
}

fn get_action() -> Action {
    loop {
        println!("Action?");
        let mut action = String::new();
        std::io::stdin().read_line(&mut action).unwrap();
        match action.trim() {
            "March" => {
                return Action::March(get_crew(""), get_zone("From"), get_zone("To"), get_u8(""))
            }
            "Battle" => {
                return Action::Battle(
                    get_crew(""),
                    get_zone("From"),
                    get_u8("Rogues"),
                    get_u8("Bullies"),
                    get_u8("Goons"),
                )
            }
            "Recruit" => return Action::Recruit(get_crew(""), get_zone("To")),
            "Negotiate" => return Action::StartNegotiation,
            _ => println!("Invalid action: {}", action),
        }
    }
}

fn main() {
    let arg = std::env::args().nth(1).unwrap();
    let players = arg.parse::<u8>().unwrap();
    let mut board = tergiversators_core::start_game(players).unwrap();
    let mut res = Ok(None);

    loop {
        show_board(&board);
        let action = get_action();
        TurnResult(board, res) = tergiversators_core::take_turn(board, action);

        if let Err(Error{action: _, reason: e}) = res {
            println!("Error: {e}");
            continue;
        }
        if matches!(action, Action::StartNegotiation) {
            show_board(&board);
            loop {
                TurnResult(board, res) = tergiversators_core::take_turn(
                    board,
                    Action::EndNegotiation(get_crew("Return")),
                );

                if let Err(Error{action: _, reason: e}) = res {
                    println!("Error: {e}");
                    continue;
                }
                break;
            }
        }
        dbg!(&board);
        if let Ok(Some(winner)) = res {
            println!("Winner: {}", match winner {
                Winner::Draw => "Draw",
                Winner::Player(Player::Alpha) => "First player",
                Winner::Player(Player::Beta) => "Second player",
                Winner::Player(Player::Gamma) => "Third player",
                Winner::Player(Player::Delta) => "Fourth player",
                Winner::Player(Player::Epsilon) => "Fifth player",
            });
            break;
        }
    }
}
