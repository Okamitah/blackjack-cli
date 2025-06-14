use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

fn main() {

    let iterations = 100_000;
    let bet_amount = 1.0;
    test_strategy(basic_strategy, 0.0, bet_amount, iterations, String::from("Basic (Stand on 17)"));
    test_strategy(aggressive_strategy, 0.0, bet_amount, iterations, String::from("Aggressive (Stand on 15)"));
    test_strategy(conservative_strategy, 0.0, bet_amount, iterations, String::from("Conservative (Stand on 18)"));
    test_strategy(perfect_game, 0.0, bet_amount, iterations, String::from("Perfect"));
}

fn test_strategy(
    strategy: impl Fn(u8, &mut Player, &Dealer) -> &'static str,
    initial_budget: f32,
    fixed_bet: f32,
    iterations: usize,
    strategy_name: String
) {
    
    let (final_budget, net_result, is_profitable) =
        simulate_blackjack(iterations, initial_budget, fixed_bet, strategy);

    println!("\n\nStrategy: {}", strategy_name);
    println!("Final budget: {:.2}", final_budget);
    println!("Net result: {}", net_result);
    println!(
        "Profitable: {}",
        if is_profitable { "Yes" } else { "No" }
    );
}

#[derive(Debug)]
struct Card {
    //suit: Suit,
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

#[derive(Debug ,Clone, Copy, PartialEq)]
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
        let mut aces = 0;

        for card in &self.cards {
            sum += card.value;
            if card.rank == Rank::Ace {
                aces += 1;
            }
        }

        while sum > 21 && aces > 0 {
            sum -= 10;
            aces -= 1;
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
                let mut card = Card {/*suit,*/rank,value:0};
                card.value = card.get_value();
                deck.push(card);
            }
        }
        let mut rng = thread_rng();
        deck.shuffle(&mut rng);
        deck
    }
}

fn start_game_cli(mut bet_amount: f32) -> f32 {

    let mut player = Player::new();
    let mut dealer = Dealer::new();

    let deck = Deck::new();
    let mut deck_cards = deck.initiate();

    player.initiate_hand(&mut deck_cards);
    dealer.initiate_hand(&mut deck_cards);

    if player.hand_value() == 21 {
        println!("Player's hand: {:?}", player.cards);
        println!("Blackjack!");
        dealer.draw(&mut deck_cards);
        if dealer.hand_value() == 21 {
            return bet_amount;
        } else {
            return bet_amount*2.5;
        }
    }
    println!("Player's hand: {}", player.hand_value());
    println!("\nDealer's hand: {}", dealer.hand_value());

    while !player.bust() && !dealer.bust() {
        println!("\n\nAction:\n\tH: hit,\n\tS: stand,\n\tD: double,\n\tSp: split");
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
                    println!("\nYou win {}!!!", bet_amount*2.0);
                    return bet_amount*2.0;
                } else if player.hand_value() < dealer.hand_value()
                && dealer.hand_value() <= 21{
                    println!("\nYou lose");
                    return 0.0;
                } else if player.hand_value() == dealer.hand_value() {
                    println!("\nIt's a draw");
                    return bet_amount;
                }
                break;
            }
            "D" => {
                bet_amount = bet_amount*2.0;
                player.hit(&mut deck_cards);
                if player.bust() {
                    return 0.0;
                } else {
                    dealer.draw(&mut deck_cards);
                    player.hand_value();
                    dealer.hand_value();
                    if player.hand_value() > dealer.hand_value() {
                        return bet_amount*2.0;
                    } else if dealer.hand_value() <= 21 {
                        return 0.0;
                    } else {
                        return bet_amount;
                    }
                }
                
            }
            "Sp" => {

        }
        _ => println!("Not an option"),
    }

    }

    if player.bust() {
        println!("\nYou lose");
        return 0.0;
    }

    if dealer.bust() {
        println!("\nYou win {}!!!", bet_amount*2.0);
        return 2.0*bet_amount;
    }

    0.1
}


fn start_session_cli(mut budget: f32) {
    println!("Welcome to the table");

    while budget > 0.0 {
        println!("\nEnter your bet (remaining amount: {})", budget);
        let mut bet_amout: String = String::new();
        io::stdin().read_line(&mut bet_amout).expect("idk man");
        let bet_amount:f32 = match bet_amout.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("idk man"),
        };
        
        if bet_amount > budget {
            println!("Lower your bet");
        } else {
            budget -= bet_amount;
            let bet_outcome = start_game_cli(bet_amount);
            budget += bet_outcome;
        }
    }
}

fn simulate_blackjack(
    iterations: usize,
    initial_budget: f32,
    fixed_bet: f32,
    strategy: impl Fn(u8, &mut Player, &Dealer) -> &'static str,
) -> (f32, f32, bool) {

    let mut budget = initial_budget;

    for i in 0..iterations {
        /*if budget < fixed_bet {
            println!("Out of money at iteration {}", i + 1);
            break;
        }*/

        budget -= fixed_bet;
        let outcome = start_game(fixed_bet, &strategy);
        budget += outcome;
        //if i%100 == 0 {println!("{}", budget);}
    }

    let net_result = budget - initial_budget;
    let is_profitable = net_result > 0.0;

    (budget, net_result, is_profitable)
}

fn start_game(
    mut bet_amount: f32,
    strategy: impl Fn(u8, &mut Player, &Dealer) -> &'static str,
) -> f32 {
    let mut player = Player::new();
    let mut dealer = Dealer::new();

    let deck = Deck::new();
    let mut deck_cards = deck.initiate();

    player.initiate_hand(&mut deck_cards);
    dealer.initiate_hand(&mut deck_cards);

    let p_val = player.hand_value();
    let d_val = dealer.hand_value();

    if p_val == 21 {
        dealer.draw(&mut deck_cards);
        if dealer.hand_value() == 21 {
            return bet_amount;
        } else {
            return bet_amount * 2.5;
        }
    }

    loop {
        let action = strategy(p_val, &mut player, &dealer);
        match action {
            "H" => {
                player.hit(&mut deck_cards);
                let new_val = player.hand_value();
                if new_val > 21 {
                    return 0.0;
                }
            }
            "S" => {
                dealer.draw(&mut deck_cards);
                let d_val = dealer.hand_value();
                let p_val = player.hand_value();

                if d_val > 21 || p_val > d_val {
                    return bet_amount * 2.0;
                } else if p_val < d_val && d_val <= 21 {
                    return 0.0;
                } else {
                    return bet_amount;
                }
            }
            "D" => {
                bet_amount *= 2.0;
                player.hit(&mut deck_cards);
                if player.bust() {
                    return 0.0;
                }

                dealer.draw(&mut deck_cards);
                let d_val = dealer.hand_value();
                let p_val = player.hand_value();

                if d_val > 21 || p_val > d_val {
                    return bet_amount;
                } else if p_val < d_val && d_val <= 21 {
                    return 0.0;
                } else {
                    return bet_amount / 2.0;
                }
            }
            _ => break,
        }
    }

    0.0
}

fn basic_strategy(player_val: u8, _: &mut Player, _: &Dealer) -> &'static str {
    if player_val < 17 {
        "H"
    } else {
        "S"
    }
}

fn conservative_strategy(player_val: u8, _: &mut Player, _: &Dealer) -> &'static str {
    if player_val < 18 {
        "H"
    } else {
        "S"
    }
}

fn aggressive_strategy(player_val: u8, _: &mut Player, _: &Dealer) -> &'static str {
    if player_val < 15 {
        "H"
    } else {
        "S"
    }
}

fn perfect_game(player_val: u8, player: &mut Player, dealer: &Dealer) -> &'static str {
    let dealer_upcard = match dealer.cards.get(0) {
        Some(card) => card.rank,
        None => panic!("Dealer has no visible card"),
    };

    let is_soft_hand = player.cards.iter().any(|c| c.rank == Rank::Ace);

    if !is_soft_hand {
        match player_val {
            5..=7 => "H",
            8 => match dealer_upcard {
                Rank::Two | Rank::Three | Rank::Four | Rank::Five | Rank::Six => "D",
                _ => "H", 
            },
            9 => match dealer_upcard {
                Rank::Two | Rank::Three | Rank::Four | Rank::Five | Rank::Six | Rank::Seven => "D",
                _ => "H",
            },
            10 => match dealer_upcard {
                Rank::Ten | Rank::Jack | Rank::Queen | Rank::King | Rank::Ace => "D",
                _ => "H",
            },
            11 => match dealer_upcard {
                Rank::Two | Rank::Three | Rank::Four | Rank::Five | Rank::Six | Rank::Seven | Rank::Eight | Rank::Nine | Rank::Ten | Rank::Jack | Rank::Queen | Rank::King | Rank::Ace => "D",
            },
            12 => match dealer_upcard {
                Rank::Four | Rank::Five | Rank::Six => "S",
                _ => "H",
            },
            13..=16 => match dealer_upcard {
                Rank::Two | Rank::Three | Rank::Four | Rank::Five | Rank::Six => "S",
                _ => "H", 
            },
            17..=21 => "S", 
            _ => "H",
        }
    } else {
        match player_val {
            13..=17 => match dealer_upcard {
                Rank::Two | Rank::Three | Rank::Four | Rank::Five | Rank::Six | Rank::Seven => "H",
                _ => "S",
            },
            18 => match dealer_upcard {
                Rank::Nine | Rank::Ten | Rank::Jack | Rank::Queen | Rank::King | Rank::Ace => "S",
                _ => "H",
            },
            19 => match dealer_upcard {
                Rank::Ten | Rank::Jack | Rank::Queen | Rank::King | Rank::Ace => "S",
                _ => "H",
            },
            20 => "S",
            21 => "S", 
            _ => "H", 
        }
    }

}
