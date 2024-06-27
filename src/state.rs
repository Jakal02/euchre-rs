use crate::card::Card;
use crate::card::Suit;
use crate::card::Rank;

pub enum FlippedChoice {
    Available,
    TurnedDown,
    PickedUp,
}

pub struct ScopedGameState {

    pub relative_dealer_player_id: usize,

    pub current_player_hand: Vec<Card>,
    pub current_player_id: usize,

    pub flipped_card: Card,
    pub flipped_choice: FlippedChoice,

    // false if in round 0 or 1, true otherwise
    pub trump_called: bool,
    // None if in round 0 or 1
    pub trump: Option<Suit>,
    // None if in round 0 or 1
    pub relative_calling_player: Option<usize>,

    // None if in round 0 or 1
    pub led_suit: Option<Suit>,
    // None if in round 0 or 1
    pub center_cards: Option<Vec<Card>>,
    // None if in round 0 or 1
    pub relative_play_order: Option<Vec<usize>>,

    // empty at beginning, grows.
    // TODO: change implementation to also store who played the card
    pub play_history: Vec<Card>,

    // dummy hidden private field. Now new method can have limited arguments.
    _private: (),
}

impl ScopedGameState {
    /// Create a ScopedGameState which represents the start of the game.
    /// Fills in guranteed defaults, requires values that are random every game.
    fn new(current_hand: Vec<Card>, ) -> ScopedGameState{

        ScopedGameState {
            // dealer always on the right at the beginning
            relative_dealer_player_id: 3,

            // BREAKING DEFAULT
            current_player_hand: vec![],
            // BREAKING DEFAULT
            current_player_id: 5,

            trump_called: false,
            trump: None,
            relative_calling_player: None,
            
            // BREAKING DEFAULT
            flipped_card: Card{suit: Suit::Spades, rank: Rank::Ace},
            flipped_choice: FlippedChoice::Available,

            led_suit: None,
            center_cards: None,
            relative_play_order: None,

            play_history: vec![],

            _private: (),
        }
    }
}

