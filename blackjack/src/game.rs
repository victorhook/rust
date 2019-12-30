use super::deck::{Deck, Player};
use std::io::{stdin};

pub fn start_game(deck: Deck, player: Player, computer: Player) {
    
    let mut deck = deck;
    let mut player = player;
    let mut computer = computer;

    let mut game_running = true;

    while game_running {
        deck.shuffle();

        player.add_card(deck.draw());
        player.add_card(deck.draw());
        computer.add_card(deck.draw());
        computer.add_card(deck.draw());

        while deck.has_more_cards() && !winner(&player, &computer) {
            display_screen(&player, &computer);
            
            if player_wants_to_move() {
                println!("Adding new card");
                player.add_card(deck.draw());
            } else {
                player.stop();
            }

            if computer_wants_to_move(&player, &computer) {
                computer.add_card(deck.draw());
            } else {
                computer.stop();
            }
        }

        announce_winner(&player, &computer);
        if !play_again() {
            println!("Thank you for playing!");
            std::process::exit(0);
        }
    }
}

fn play_again() -> bool {
    let mut play_again = String::new();
    print!("Do you want to play again? yes/no");
    stdin().read_line(&mut play_again)
            .expect("Error");

    match play_again.to_lowercase().as_ref() {
        "yes" => true,
        "y" => true,
        _ => false,
    }
}

fn announce_winner(player: &Player, computer: &Player) {
    display_screen(&player, &computer);
    let p_score = player.get_score();
    let c_score = computer.get_score();
    if p_score < 21 && p_score > c_score {
        println!("{} has won!", player.get_name());
    } else if c_score < 21 && c_score > p_score {
        println!("{} has won!", computer.get_name());
    } else {
        println!("It's a tie!");
    }
}

fn winner(player: &Player, computer: &Player) -> bool {
    player.has_stopped() && computer.has_stopped()
}

fn display_screen(player: &Player, computer: &Player) {
    println!("\n");

    print!("      == {}  Score: {} == \nHand:", player.get_name(), player.get_score());
    player.show_hand();
    println!("\n");

    print!("      == {}  Score: {} == \nHand:", computer.get_name(), computer.get_score());
    computer.show_hand();
    println!("\n");
}

// TODO: FIX THE MATCH
fn player_wants_to_move() -> bool {
    let mut player_move = String::new();
    
    println!("Do you want to hit or stay?\n");
    stdin().read_line(&mut player_move)
            .expect("Error");

    let m = match player_move.to_lowercase().as_str() {
        "hit" => true,
        "h" => true,
        "stay" => true,
        "s" => true,
        _ => false
    };
    println!("{}", m);
    return m;
}

fn computer_wants_to_move(player: &Player, computer: &Player) -> bool {
    let p_score = player.get_score();
    let c_score = computer.get_score();
    p_score > c_score && c_score < 21 && p_score < 21 ||
    c_score < 15 && p_score < 21
}