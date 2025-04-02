use std::io;
use rand::Rng;

// Struct for defining a card
struct Card {
    suit: String,
    face: String,
    weight: u8
}

// Struct for defining a player
struct Player {
    current_hand: Vec<Card>,
    collected_hand: Vec<Card>
}

// Implementations for Card struct
impl Card {

    // Getters for all Card values
    fn get_suit(&self) -> &String {
        &self.suit
    }

    fn get_face(&self) -> &String {
        &self.face
    }

    fn get_weight(&self) -> u8 {
        self.weight
    }
}

// Implementations for Player struct
impl Player {

    fn get_current_hand_count(&self) -> usize {
        self.current_hand.len()
    }

    fn get_collected_hand_count(&self) -> usize {
        self.collected_hand.len()
    }

    fn check_if_lost(&self) -> bool {
        if self.get_current_hand_count() + self.get_collected_hand_count() == 0 {
            return true
        }
        false
    }
}

fn main() {

    // Create vector of Card type to initialise the deck with
    let deck: Vec<Card> = create_deck();
    let shuffled_deck: Vec<Card> = shuffle_deck(deck);
    let (p1_start, p2_start) = split_deck(shuffled_deck);

    // Create the players
    let player_1: Player = Player {
        current_hand: p1_start,
        collected_hand: Vec::new()
    };

    let player_2: Player = Player {
        current_hand: p2_start,
        collected_hand: Vec::new()
    };

    // Start game
    game_loop(player_1, player_2);
}

// Creates a non shuffled deck of cards
fn create_deck() -> Vec<Card> {

    // Create vector of Card type, and two fixed sized arrays representing the card Suits and Faces
    let mut deck: Vec<Card> = Vec::new();
    let suits: [&str; 4] = ["Diamonds", "Clubs", "Hearts", "Spades"];
    let faces: [&str; 13] = ["Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King", "Ace"];

    for outer in 0..4 {                                         // Loop for each suit

        for inner in 0..13 {                                      // Loop for each face

            let weight: u8 = inner + 1;

            let card: Card = Card {                                   // Create card and push into the deck
                suit: String::from(suits[outer]),
                face: String::from(faces[inner as usize]),
                weight,
            };

            deck.push(card);
        }
    }

    deck
}

fn shuffle_deck(mut deck: Vec<Card>) -> Vec<Card> {

    let mut shuffled_deck: Vec<Card> = Vec::new();

    while !deck.is_empty() {
        let random_index: usize = rand::rng().random_range(0..=deck.len() - 1);
        let current_card: Card = deck.remove(random_index);
        shuffled_deck.push(current_card);
    }

    shuffled_deck
}

fn split_deck(mut deck: Vec<Card>) -> (Vec<Card>, Vec<Card>) {
    let mut first_half: Vec<Card> = Vec::new();
    let mut second_half: Vec<Card> = Vec::new();
    let deck_length: usize = deck.len();

    // Split shuffled deck down the middle into two starting hands
    for outer in 0..2 {
        for _inner in 0..(deck_length / 2) {
            let current_card: Option<Card> = deck.pop();
            match current_card {
                None => { std::process::exit(0) }
                Some(x) => {
                    if outer == 0 {
                        first_half.push(x);
                    } else {
                        second_half.push(x);
                    }
                }
            }
        }
    }

    (first_half, second_half)
}

fn war_winner(p1_card: Card, p2_card: Card, winner: &mut Player, loser: &mut Player, index: usize) {
    winner.collected_hand.push(p1_card);
    winner.collected_hand.push(p2_card);
    for _i in 0..index + 1 {
        let temp_card: Option<Card> = winner.current_hand.pop();
        match temp_card {
            None => { std::process::exit(0) },
            Some(x) => {
                winner.collected_hand.push(x);
            }
        }
        let temp_card: Option<Card> = loser.current_hand.pop();
        match temp_card {
            None => { std::process::exit(0) },
            Some(x) => {
                winner.collected_hand.push(x);
            }
        }
    }
}

fn game_loop(mut p1: Player, mut p2: Player) {

    loop {
        println!("P1 Current Hand: {}          P2 Current Hand: {}", p1.get_current_hand_count(), p2.get_current_hand_count());
        println!("P1 Collected Hand: {}        P2 Collected Hand: {}", p1.get_collected_hand_count(), p2.get_collected_hand_count());
        println!("There are {} cards in the deck", p1.current_hand.len() + p1.collected_hand.len() + p2.current_hand.len() + p2.collected_hand.len());
        println!("-----------------------------------------------------");
        println!("Press enter to play the next turn!");

        let mut play: String = String::new();
        io::stdin().read_line(&mut play).expect("");

        let p1_card: Card = p1.current_hand.remove(p1.current_hand.len() - 1);
        let p2_card: Card = p2.current_hand.remove(p2.current_hand.len() - 1);

        println!("Player 1 Played: {} of {}", p1_card.get_face(), p1_card.get_suit());
        println!("Player 2 Played: {} of {}", p2_card.get_face(), p2_card.get_suit());

        if p1_card.get_weight() > p2_card.get_weight() {
            p1.collected_hand.push(p1_card);
            p1.collected_hand.push(p2_card);
        } else if p1_card.get_weight() < p2_card.get_weight() {
            p2.collected_hand.push(p1_card);
            p2.collected_hand.push(p2_card);
        } else {
            println!("GOING TO WAR!!");
            let mut index: usize = 3;
            loop {
                if p1.current_hand.len() < index + 1 && p2.current_hand.len() < index + 1 {
                    println!("It's a Draw!");
                    std::process::exit(0);
                }
                if p1.current_hand.len() < index + 1 {
                    println!("Player 2 Wins!");
                    std::process::exit(0);
                }
                if p2.current_hand.len() < index + 1 {
                    println!("Player 1 Wins!");
                    std::process::exit(0);
                }

                if p1.current_hand[index].get_weight() > p2.current_hand[index].get_weight() {
                    war_winner(p1_card, p2_card, &mut p1, &mut p2, index);
                    break;
                }

                if p1.current_hand[index].get_weight() < p2.current_hand[index].get_weight() {
                    war_winner(p1_card, p2_card, &mut p2, &mut p1, index);
                    break;
                }

                index += 4;
            }
        }

        if p1.current_hand.len() == 0 {
            let lost: bool = p1.check_if_lost();
            if lost == true {
                println!("Player 2 Wins!");
                std::process::exit(0);
            } else {
                let collected_length: usize = p1.collected_hand.len();
                p1.collected_hand = shuffle_deck(p1.collected_hand);
                for _num in 0..collected_length {
                    let temp_card: Option<Card> = p1.collected_hand.pop();
                    match temp_card {
                        None => { std::process::exit(0) },
                        Some(x) => {
                            p1.current_hand.push(x);
                        }
                    }
                }
            }
        }

        if p2.current_hand.len() == 0 {
            let lost: bool = p2.check_if_lost();
            if lost == true {
                println!("Player 1 Wins!");
                std::process::exit(0);
            } else {
                let collected_length: usize = p2.collected_hand.len();
                p2.collected_hand = shuffle_deck(p2.collected_hand);
                for _num in 0..collected_length {
                    let temp_card: Option<Card> = p2.collected_hand.pop();
                    match temp_card {
                        None => { std::process::exit(0) },
                        Some(x) => {
                            p2.current_hand.push(x);
                        }
                    }
                }
            }
        }
    }
}