fn main() {
    println!("Hello, world!");
    start_game();
}

#[derive(Debug)]
struct Card {
    suit: Suit,
    rank: String,
    value: u8,
}

 #[derive(Debug)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
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

    fn hit(&self) {
    }
}


fn create_deck() -> [Card; 52] {
    core::array::from_fn(|i| {
        let suit = match i%4 {
            0 => Suit::Hearts ,
            1 => Suit::Clubs,
            2 => Suit::Spades,
            3 => Suit::Diamonds,
            _ => Suit::Clubs,
        };
        let rank_val:u8 = ((i/4)%13+1).try_into().unwrap();
        let rank: String = rank_val.to_string();
        Card{suit,rank, value:rank_val}
    })
}

fn start_game() {
    let deck = create_deck();

}

fn player_bust(player_total: u8) -> bool {
    if player_total > 21 {return true;}
    false
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

fn hit(player: Player, deck: Vec<Card>) {
    
}

