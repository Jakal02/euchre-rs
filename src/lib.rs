use rand::Rng;
use rand::prelude::ThreadRng;

pub mod state;
use state::ScopedGameState;
pub mod player;
use player::{Player, Strategy};
pub mod card;
use card::Card;
pub mod deck;
use deck::Deck;
pub mod action;

/// Function to run Euchre.
pub fn run() {
    println!("This is Euchre!");

    // Set up Game
    let mut deck: Deck = Deck::new();
    deck.shuffle();

    let players: Vec<Player> = vec![
        Player {hand: deck.deal_n_cards(5)},
        Player {hand: deck.deal_n_cards(5)},
        Player {hand: deck.deal_n_cards(5)},
        Player {hand: deck.deal_n_cards(5)}
    ];

    let dealer_ind: usize = determine_dealer();

    // let p1_strat: &Strategy = & players[dealer_ind].strategy;

    println!("You are player 0 \n\
            The Dealer is player {:?}.\n",
            dealer_ind);

    println!("Your hand is: {}\n-------------\n", players[0]);


    // initialize game state
    let mut state: ScopedGameState = ScopedGameState{..Default::default()};

    // Rounds 0-1 - Deciding Trump
    let mut trump_decided: bool = false;
    let mut current_player: usize = increment_player(dealer_ind);
    let flipped_card: Card = (deck.deal_n_cards(1)).pop().unwrap();

    // Round 0
    for _ in 0..3 {

    }


}

pub fn determine_dealer() -> usize {
    let mut rng: ThreadRng = rand::thread_rng();
    rng.gen_range(0..=3)
}

/// Return the index of the player one greater than the current player.
/// The index of 3 loops back to zero.
pub fn increment_player(current_player: usize) -> usize {
    (current_player + 1) % 4
}