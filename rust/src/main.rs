fn main() {
    let mut pl = Player {
        cards: vec![
            Card{suit: Suit::Clubs, rank: Rank::Ace, value: 11},
            Card{suit: Suit::Clubs, rank: Rank::Jack, value: 10}
        ]
    };
    let vl = pl.hand_value();
    println!("Hand's value: {:?}", vl);
    let mut deck = Deck{cards:Vec::new()};
    let mut deck_cards = deck.initiate();
    pl.hit(deck_cards);
    let vl = pl.hand_value();
    println!("Hand's value: {:?}", vl);

}

#[derive(Debug)]
struct Card {
    suit: Suit,
    rank: Rank,
    value: u8,
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
                deck.push(Card{suit, rank, value: 5});
            }
        }
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
