use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}

impl Deck {
    fn new() -> Self {
        let mut cards = vec![];
        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];
    
        for suit in suits {
            for value in values {
                let card = format!("{value} of {suit}");
                cards.push(card);
            }
        }
        
        Deck { cards }
    }

    fn shuffle(&mut self) {
         let mut rng = thread_rng();
         self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }

}

fn main() {
    let mut deck: Deck = Deck::new();
    // deck.shuffle();
    let cards_on_hand = deck.deal(3);
    
    println!("here is your deck: {:#?}", deck);
    println!("here is your hand: {:#?}", cards_on_hand);
}
