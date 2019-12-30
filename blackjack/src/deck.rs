use rand::Rng;
use std::fmt;

const SUITS: [&'static str; 4] =  ["Hearts","Diamonds", "Clubs", "Spades"];

pub struct Player {
    name: String,
    hand: Vec<Card>,
    score: usize,
    stop: bool
}

impl Player {

    pub fn new(name: String) -> Self {
        Player {
            name: name,
            hand: Vec::new(),
            score: 0,
            stop: false,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_score(&self) -> usize {
        self.score
    }

    pub fn clear_hand(&mut self) {
        self.hand.clear();
        self.score = 0;
        self.stop = false;
    }

    /* Adds the given card to the player hand and updates
        the score. (If the card is Jack, Queen, King or Ace
        it's value is 10.) */
    pub fn add_card(&mut self, card: Card) {
        self.hand.push(card);
        let value = match card.rank {
            x if x > 10 => 10,
            _ => card.rank,
        };
        self.score += value;
    }

    pub fn show_hand(&self) {
        for card in &self.hand {
            print!(" | {} | ", card);
        }
    }

    /* Returns wether the player wants to hit or not */
    pub fn has_stopped(&self) -> bool {
        self.stop
    }

    /* Makes the player stop hitting */
    pub fn stop(&mut self) {
        self.stop = true;
    }

}


#[derive(Copy, Clone)]
pub struct Card {
    suit: usize,
    rank: usize,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.suit == other.suit && self.rank == other.rank
    }
}

/* Gives the card a nice printable format.
    It is probably really not the smoothest way to
    implement it but but it works! */
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut rank: Option<&str> = match self.rank {
            11 => Some("Jack"),
            12 => Some("Queen"),
            13 => Some("King"),
            1 => Some("Ace"),
            _ => None,
        };

        if rank.is_some() {
            return write!(f, "{} of {}", rank.unwrap(), SUITS[self.suit]);
        } else {
            write!(f, "{} of {}", self.rank, SUITS[self.suit])
        }
    }
}


pub struct Deck {
    deck: [Card; 52],
    cards: usize,
    rand: rand::ThreadRng,
}

impl Deck {

    pub fn new() -> Deck {
        let mut deck = Deck {
                        deck: [Card {suit: 0, rank: 0}; 52],
                        cards: 0,
                        rand: rand::thread_rng(),
                    };
        
        for suit in 0..4 {
            for rank in 1..14 {
                deck.deck[deck.cards].suit = suit;
                deck.deck[deck.cards].rank = rank;
                deck.cards += 1;
            }
        }
        return deck;
    }

    pub fn print(&self) {
        for card in 0..self.cards {
            if card % 10 == 0 && card != 0 {
                println!();
            }
            print!(" | {} | ", self.deck[card]);
        };
        println!("\n");
    }

    /* Shuffles the deck and resets the card counter so
        it's as good as a new deck */
    pub fn shuffle(&mut self){
        for i in (1..self.cards).rev() {
            let random_index = self.rand.gen_range(0, i + 1);
            let temp = self.deck[i];
            self.deck[i] = self.deck[random_index];
            self.deck[random_index] = temp;
        }
        self.cards = 52;
    }

    pub fn draw(&mut self) -> Card {
        let random_card = self.deck[self.rand.gen_range(0, self.cards)];
        self.pop(random_card);
        self.cards -= 1;
        return random_card;
    }

    pub fn pop(&mut self, card_to_remove: Card) {
        let mut card = 0;
        while card < self.cards {
            if self.deck[card] == card_to_remove {
                while card < self.cards - 1 {
                    self.deck[card] = self.deck[card + 1];
                    card += 1;
                }
                break;
            }
            card += 1;
        }
    }

    pub fn has_more_cards(&self) -> bool {
        self.cards > 0
    }

}

