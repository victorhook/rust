use rand::Rng;
use std::fmt;

const SUITS: [&'static str; 4] =  ["Hearts","Diamonds", "Clubs", "Spades"];

#[derive(Copy, Clone)]
struct Card {
    suit: usize,
    rank: usize,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.suit == other.suit && self.rank == other.rank
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.rank, SUITS[self.suit])
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
            print!("{}, ", self.deck[card]);
        };
        println!("\n");
    }

    pub fn shuffle(mut self) -> Deck {
        for i in (1..self.cards).rev() {
            let random_index = self.rand.gen_range(0, i);
            let temp = self.deck[i];
            self.deck[i] = self.deck[random_index];
            self.deck[random_index] = temp;
        }
        return self;
    }

    pub fn draw(mut self) -> (Deck, Card) {
        let random_card = self.deck[self.rand.gen_range(0, self.cards)];
        self = self.pop(random_card);
        self.cards -= 1;
        return (self, random_card);
    }

    fn pop(mut self, card_to_remove: Card) -> Deck {
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
        return self;
    }

}

pub fn run() {

    let mut deck = Deck::new();
    let mut drawn_card: Card;

    deck.print();
    let (deck, drawn_card) = deck.draw();
    println!("{}", drawn_card);
    deck.print();

}