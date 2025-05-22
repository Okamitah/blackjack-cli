use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

fn main() {
    start_game();
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
    fn new() -> Self {
        Self { cards: Vec::new() }
    }

    fn hand_value(&self) -> u8 {
        let mut sum = 0;
        for card in &self.cards {
            sum += card.value;
        }
        sum
    }

    fn initiate_hand(&mut self, deck: &mut Vec<Card>) {
        self.hit(deck);
        self.hit(deck);
    }

    fn hit(&mut self, deck: &mut Vec<Card>) {
        self.cards.push(deck.pop().expect("Empty deck lil asaf"));
    }

    fn bust(&self) -> bool {
        if *&self.hand_value() > 21 {return true;}
        false
    }
}

struct Dealer {
    cards: Vec<Card>,
}

impl Dealer {
    fn new() -> Self {
        Self {cards: Vec::new()}
    }

    fn hand_value(&self) -> u8 {
        let mut sum = 0;
        for card in &self.cards {
            sum += card.value;
        }
        sum
    }

    fn initiate_hand(&mut self, deck: &mut Vec<Card>) {
        self.hit(deck);
    }

    fn draw(&mut self, deck: &mut Vec<Card>) {
        while self.hand_value() < 17 {
            self.hit(deck);
        }
    }

    fn hit(&mut self, deck: &mut Vec<Card>) {
        self.cards.push(deck.pop().expect("Empty deck lil asaf"));
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
    fn new() -> Self {
        Self {cards: Vec::new()}
    }

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

    let mut player = Player::new();
    let mut dealer = Dealer::new();

    let deck = Deck::new();
    let mut deck_cards = deck.initiate();

    player.initiate_hand(&mut deck_cards);
    dealer.initiate_hand(&mut deck_cards);

    let pl_hand_value = player.hand_value();
    let dl_hand_value = dealer.hand_value();

    println!("Player's hand: {}", pl_hand_value);
    println!("\nDealer's hand: {}", dl_hand_value);

    while !player.bust() && !dealer.bust() {
        println!("\n\nAction:\n\tH: hit,\n\tS: stand");
        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("idk man");

        match action.as_str().trim() {
            "H" => {
                player.hit(&mut deck_cards);
                println!("Player's hand: {}", player.hand_value());
                println!("\nDealer's hand: {}", dealer.hand_value());
            }
            "S" => {
                dealer.draw(&mut deck_cards);
                println!("Player's hand: {}", player.hand_value());
                println!("\nDealer's hand: {}", dealer.hand_value());
                if player.hand_value() > dealer.hand_value() {
                    println!("You win!!!");
                } else if player.hand_value() < dealer.hand_value() {
                    println!("You lose");
                } else {
                    println!("It's a draw");
                }
                break;
            }
            _ => println!("Not an option"),
        }

    }

    if player.bust() {
        println!("You lose");
    }

    if dealer.bust() {
        println!("You win!!!");
    }

}

