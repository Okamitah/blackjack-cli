use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut pl = Player {
        cards: vec![
            Card{suit: Suit::Clubs, rank: Rank::Two, value: 2},
            Card{suit: Suit::Clubs, rank: Rank::Jack, value: 10}
        ]
    };
    let vl = pl.hand_value();
    println!("Hand's value: {:?}", vl);
    let deck = Deck{cards:Vec::new()};
    let deck_cards = deck.initiate();
    pl.hit(deck_cards);
    println!("Player's hand: {:?}", pl.cards);
    let vl = pl.hand_value();
    if pl.bust() {
        println!("You're bust :(, value: {}", vl);
    } else {
        println!("Your hand: {}", vl);
    }

}

#[derive(Debug)]
struct Card {
    suit: Suit,
    rank: Rank,
    value: u8,
}

impl Card {
    fn get_value(&self) -> u8 {
        match &self.rank {
            Rank::Ace => 11,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack=> 10,
            Rank::Queen => 10,
            Rank::King => 10,
        }
    }
}

 #[derive(Debug)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

impl Suit {
    fn iter() -> impl Iterator<Item=Self> {
        [
            Self::Clubs,
            Self::Diamonds,
            Self::Hearts,
            Self::Spades,
        ]
        .into_iter()
    }
}

#[derive(Debug ,Clone, Copy)]
enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Rank {
    fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Ace,
            Self::Two,
            Self::Three,
            Self::Four,
            Self::Five,
            Self::Six,
            Self::Seven,
            Self::Eight,
            Self::Nine,
            Self::Ten,
            Self::Jack,
            Self::Queen,
            Self::King,
        ]
            .into_iter()
    }
}

#[derive(Debug)]
struct Player {
    cards: Vec<Card>,
}

impl Player {
    fn hand_value(&self) -> u8 {
        let mut sum = 0;
        for card in &self.cards {
            sum += card.value;
        }
        sum
    }

    fn hit(&mut self, mut deck: Vec<Card>) {
        &self.cards.push(deck.pop().expect("Empty deck lil asaf"));
    }

    fn bust(&self) -> bool {
        if *&self.hand_value() > 21 {return true;}
        false
    }
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn initiate(&self) -> Vec<Card> {
        let mut deck = Vec::new();
        for rank in Rank::iter() {
            for suit in Suit::iter() {
                let mut card = Card {suit,rank,value:0};
                card.value = card.get_value();
                deck.push(card);
            }
        }
        let mut rng = thread_rng();
        deck.shuffle(&mut rng);
        deck
    }
}

fn start_game() {

}

fn dealer_bust(dealer_total: u8) -> bool {
    if dealer_total > 21 {return true;}
    false
}

fn dealer_hasto_play(dealer_total: u8) -> bool {
    if dealer_total < 17 {return true;}
    false
}

fn game_end(player_total: u8, dealer_total: u8) -> bool {
    if player_total > 21 || dealer_total > 21 {return true;}
    else {return false;}
}
