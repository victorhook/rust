mod deck;

#[derive(Copy, Clone, Debug)]
pub struct Card {
    suit: usize,
    rank: usize,
}

pub struct Deck {
    cards: usize,           // Holds the total ammount of cards in the deck
    deck: [Card; 52],       // Contains all the cards, using the cards value to know how many are relevant
    suits: [String; 4],     // Used to display the correct string according to the suits of the cards
    rand: rand::ThreadRng,  // Used for shuffling the cards
}

/*
impl Deck {

    pub fn new() -> Deck {
        let mut deck = Deck {
            cards: 0,
            deck: [Card {suit:0, rank:0}; 52],
            suits: [String::from("Hearts"), String::from("Diamonds"), 
                    String::from("Spades"), String::from("Clubs")],
            rand: rand::thread_rng(),
            
        };
        deck.make_deck();
        return deck;
    }

    fn make_deck(&mut self) {
        for suit in 0..3 {
            for rank in 1..14 {
                self.deck[self.cards].suit = suit;
                self.deck[self.cards].rank = rank;
                self.cards += 1;
            }
        }
    }

    pub fn print(self) {
        for i in 0..self.cards {
            print!(" | {} of {} | ", self.suits[self.deck[i].suit], self.deck[i].rank);
            if i % 8 == 0 && i != 0 {
                println!();
            }
        }
    }

    fn shuffle_deck(&self, deck: &mut [Card]) {

    }
    /*
    for i in self.cards..1 {
        let random_index = self.rand.gen_range(0, i);
        let temp: Card = self.deck[i];
        self.deck[i] = self.deck[random_index];
        self.deck[random_index] = temp;
    }
    return self;
    */
    pub fn shuffle(self) {
        self.shuffle_deck(&mut self.deck);
    }

    fn p(&self) {

    }

}
*/

fn main() {

    deck::run();

}
