fn main() {
    println!("Hello, world!");
}

struct Card {
    suit: Suit,
    rank: u8,
}

enum Suit {
    hearts,
    diamonds,
    spades,
    clubs,
}

fn create_deck() -> [Card; 52] {
    core::array::from_fn(|i| {
        let suit = match i%4 {
            0 => Suit::hearts ,
            1 => Suit::clubs,
            2 => Suit::spades,
            3 => Suit::diamonds,
        };
        let rank:u8 = ((i/4)%13+1).try_into().unwrap();
        Card{suit,rank}
    })
}

fn start_game(nbr_players: u8) {
    let deck = create_deck();

}
