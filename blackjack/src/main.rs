mod game;
mod deck;

fn main() {

    let mut deck = deck::Deck::new();
    let mut player = deck::Player::new("Victor".to_string());
    let mut computer = deck::Player::new("Computer".to_string());


    game::start_game(deck, player, computer);

}
