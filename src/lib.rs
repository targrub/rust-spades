//! This crate provides an implementation of the four person card game, [spades](https://www.pagat.com/auctionwhist/spades.html).
//! ## Example usage
//! ```
//! extern crate rand;
//! extern crate uuid;
//! extern crate spades;
//!
//! use spades::{Game, State, Bet};
//! use rand::{thread_rng, Rng};
//!
//! let mut g = Game::new(uuid::Uuid::new_v4(),
//!    [uuid::Uuid::new_v4(),
//!     uuid::Uuid::new_v4(),
//!     uuid::Uuid::new_v4(),
//!     uuid::Uuid::new_v4()],
//!     500);
//!
//! g.start_game();
//!
//! while g.state() != State::GameCompleted {
//!     let mut rng = thread_rng();
//!     if let State::Trick(_playerindex) = g.state() {
//!         assert!(g.current_hand().is_ok());
//!         let hand = g.current_hand().ok().unwrap().clone();
//!
//!         let random_card = rng.choose(hand.as_slice()).unwrap();
//!         
//!         g.play_card(random_card.clone());
//!     } else {
//!         g.place_bet(Bet::Amount(3));
//!     }
//! }
//! if g.is_over() {
//!     println!("All rounds of the game are complete.  The winning score was ");
//! }
//! ```

extern crate uuid;

mod cards;
mod game_state;
mod result;
mod scoring;

#[cfg(test)]
mod tests;

pub use cards::{Card, Suit};
pub use game_state::State;
pub use result::SpadesError;
pub use scoring::Bet;

use uuid::Uuid;

use result::{TransitionError, TransitionSuccess};
use scoring::Scoring;

enum GameAction {
    Bet(Bet),
    Card(Card),
    Start,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Player {
    id: Uuid,
    hand: Vec<Card>,
}

impl Player {
    fn new(id: Uuid) -> Player {
        Player { id, hand: vec![] }
    }
}

/// Primary game state. Internally manages player rotation, scoring, and cards.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Game {
    id: Uuid,
    state: State,
    scoring: Scoring,
    current_player_index: usize,
    deck: Vec<cards::Card>,
    hands_played: Vec<[Option<cards::Card>; 4]>,
    bets_placed: [Bet; 4],
    leading_suit: Option<Suit>,
    spades_broken: bool,
    //rule_blind_nil_allowed: bool,
    player: [Player; 4],
}

impl Default for Game {
    fn default() -> Self {
        Game {
            id: Uuid::default(),
            state: State::GameNotStarted,
            scoring: scoring::Scoring::default(),
            current_player_index: 0,
            deck: cards::new_deck(),
            leading_suit: None,
            spades_broken: false,
            hands_played: vec![[None; 4]],
            bets_placed: [Bet::Amount(0); 4],
            player: [
                Player::default(),
                Player::default(),
                Player::default(),
                Player::default(),
            ],
        }
    }
}

impl Game {
    pub fn new(id: Uuid, player_ids: [Uuid; 4], max_points: i32) -> Game {
        Game {
            id,
            state: State::GameNotStarted,
            scoring: scoring::Scoring::new(max_points),
            hands_played: vec![[None; 4]],
            bets_placed: [Bet::Amount(0); 4],
            deck: cards::new_deck(),
            current_player_index: 0,
            leading_suit: None,
            spades_broken: false,
            //rule_blind_nil_allowed: false,
            player: [
                Player::new(player_ids[0]),
                Player::new(player_ids[1]),
                Player::new(player_ids[2]),
                Player::new(player_ids[3]),
            ],
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// See [`State`](enum.State.html)
    pub fn state(&self) -> State {
        self.state
    }

    pub fn team_a_game_score(&self) -> Result<i32, SpadesError> {
        match (&self.state, self.current_player_index) {
            (State::GameNotStarted, _) => Err(SpadesError::GameNotStarted),
            _ => Ok(self.scoring.team[0].game_points()),
        }
    }

    pub fn team_b_game_score(&self) -> Result<i32, SpadesError> {
        match (&self.state, self.current_player_index) {
            (State::GameNotStarted, _) => Err(SpadesError::GameNotStarted),
            _ => Ok(self.scoring.team[1].game_points()),
        }
    }

    pub fn team_a_tricks(&self) -> Result<u8, SpadesError> {
        match self.state {
            State::GameNotStarted => Err(SpadesError::GameNotStarted),
            _ => Ok(self.scoring.team[0].tricks_won()),
        }
    }

    pub fn team_b_tricks(&self) -> Result<u8, SpadesError> {
        match self.state {
            State::GameNotStarted => Err(SpadesError::GameNotStarted),
            _ => Ok(self.scoring.team[1].tricks_won()),
        }
    }

    pub fn team_a_bags(&self) -> Result<u8, SpadesError> {
        match self.state {
            State::GameNotStarted => Err(SpadesError::GameNotStarted),
            _ => Ok(self.scoring.team[0].game_bags()),
        }
    }

    pub fn team_b_bags(&self) -> Result<u8, SpadesError> {
        match self.state {
            State::GameNotStarted => Err(SpadesError::GameNotStarted),
            _ => Ok(self.scoring.team[1].game_bags()),
        }
    }

    /// Returns `SpadesError` when the current game is not in the Betting or Trick stages.
    pub fn current_player_id(&self) -> Result<&Uuid, SpadesError> {
        match (&self.state, self.current_player_index) {
            (State::GameNotStarted, _) => Err(SpadesError::GameNotStarted),
            (State::GameCompleted, _) => Err(SpadesError::GameCompleted),
            (State::Betting(_), p) | (State::Trick(_), p) => Ok(&self.player[p].id),
        }
    }

    /// Returns a `SpadesError::InvalidUuid` if the game does not contain a player with the given `Uuid`.
    pub fn hand_from_player_id(&self, player_id: Uuid) -> Result<&Vec<Card>, SpadesError> {
        if player_id == self.player[0].id {
            return Ok(&self.player[0].hand);
        }
        if player_id == self.player[1].id {
            return Ok(&self.player[1].hand);
        }
        if player_id == self.player[2].id {
            return Ok(&self.player[2].hand);
        }
        if player_id == self.player[3].id {
            return Ok(&self.player[3].hand);
        }

        Err(SpadesError::InvalidUuid)
    }

    pub fn current_hand(&self) -> Result<&Vec<Card>, SpadesError> {
        match (&self.state, self.current_player_index) {
            (State::GameNotStarted, _) => Err(SpadesError::GameNotStarted),
            (State::GameCompleted, _) => Err(SpadesError::GameCompleted),
            (State::Betting(_), p) | (State::Trick(_), p) => Ok(&self.player[p].hand),
        }
    }

    pub fn leading_suit(&self) -> Result<Option<Suit>, SpadesError> {
        match &self.state {
            State::GameNotStarted => Err(SpadesError::GameNotStarted),
            State::GameCompleted => Err(SpadesError::GameCompleted),
            State::Trick(_) => Ok(self.leading_suit),
            _ => Err(SpadesError::InternalError),
        }
    }

    /// Returns an array with (only if in the trick stage).
    pub fn current_trick_cards(&self) -> Result<&[Option<cards::Card>; 4], SpadesError> {
        match self.state {
            State::GameNotStarted => Err(SpadesError::GameNotStarted),
            State::GameCompleted => Err(SpadesError::GameCompleted),
            State::Betting(_) => Err(SpadesError::GameCompleted),
            State::Trick(_) => Ok(self.hands_played.last().unwrap()),
        }
    }

    pub fn winner_ids(&self) -> Result<(&Uuid, &Uuid), SpadesError> {
        match self.state {
            State::GameCompleted => {
                if self.scoring.team[0].cumulative_points()
                    <= self.scoring.team[1].cumulative_points()
                {
                    Ok((&self.player[0].id, &self.player[2].id))
                } else {
                    Ok((&self.player[1].id, &self.player[3].id))
                }
            }
            _ => Err(SpadesError::GameNotCompleted),
        }
    }

    pub fn bets_placed(&self) -> Result<[Bet; 4], SpadesError> {
        Ok(self.bets_placed)
    }

    pub fn start_game(&mut self) {
        match self.execute_game_action(GameAction::Start) {
            Ok(TransitionSuccess::Start) => { /* started game successfully */ }
            Ok(_) => {
                panic!("unexpected action");
            }
            Err(_) => {
                panic!("error");
            }
        }
    }

    pub fn place_bet(&mut self, bet: Bet) -> Result<(), SpadesError> {
        match self.execute_game_action(GameAction::Bet(bet)) {
            Ok(TransitionSuccess::Bet) => {
                /* made bet successfully */
                Ok(())
            }
            Ok(TransitionSuccess::BetComplete) => {
                /* last bet was made successfully */
                Ok(())
            }
            Err(TransitionError::GameAlreadyStarted) => Err(SpadesError::ImproperGameStage),
            Err(TransitionError::BetInCompletedGame) => Err(SpadesError::GameCompleted),
            Err(TransitionError::BetNotInBettingStage) => Err(SpadesError::ImproperGameStage),
            _ => Err(SpadesError::InternalError),
        }
    }

    pub fn play_card(&mut self, card_played: Card) -> Result<(), SpadesError> {
        match self.execute_game_action(GameAction::Card(card_played)) {
            Ok(TransitionSuccess::PlayCard) => {
                /* card played successfully */
                Ok(())
            }
            Ok(TransitionSuccess::Trick) => {
                /* card played successfully; trick now over */
                Ok(())
            }
            Ok(TransitionSuccess::GameOver) => {
                /* card played successfully; game now over */
                Ok(())
            }
            Err(TransitionError::CardIncorrectSuit) => Err(SpadesError::CardIncorrectSuit),
            Err(TransitionError::CardNotInHand) => Err(SpadesError::CardNotInHand),
            Err(TransitionError::CardInBettingStage) => Err(SpadesError::ImproperGameStage),
            Err(TransitionError::CardInCompletedGame) => Err(SpadesError::GameCompleted),
            _ => Err(SpadesError::InternalError),
        }
    }

    /// The primary function used to progress the game state.
    /// The stages and player rotations are managed internally.
    /// The order of `GameAction` arguments should be:
    /// Start -> Bet * 4 -> Card * 4 * 13 -> Bet * 4 -> Card * 4 * 13 -> Bet * 4 -> ...
    fn execute_game_action(
        &mut self,
        entry: GameAction,
    ) -> Result<TransitionSuccess, TransitionError> {
        match entry {
            GameAction::Bet(bet) => match self.state {
                State::GameNotStarted | State::Trick(_) | State::GameCompleted => {
                    Err(TransitionError::BetNotInBettingStage)
                }
                State::Betting(rotation_status) => {
                    self.scoring.add_bet(self.current_player_index, bet);
                    if rotation_status == 3 {
                        self.scoring.betting_over();
                        self.state = State::Trick((rotation_status + 1) % 4);
                        self.current_player_index = 0;
                        return Ok(TransitionSuccess::BetComplete);
                    } else {
                        self.current_player_index = (self.current_player_index + 1) % 4;
                        self.state = State::Betting((rotation_status + 1) % 4);
                    }

                    Ok(TransitionSuccess::Bet)
                }
            },
            GameAction::Card(card) => {
                match self.state {
                    State::GameNotStarted => Err(TransitionError::GameNotStarted),
                    State::GameCompleted => Err(TransitionError::CardInCompletedGame),
                    State::Betting(_rotation_status) => Err(TransitionError::CardInBettingStage),
                    State::Trick(rotation_status) => {
                        {
                            let player_hand = &mut self.player[self.current_player_index].hand;

                            if !player_hand.contains(&card) {
                                return Err(TransitionError::CardNotInHand);
                            }
                            let leading_suit = self.leading_suit;
                            if rotation_status == 0 {
                                self.leading_suit = Some(card.suit);
                                // to lead spades, spades must be broken OR only have spades in this hand
                                if card.suit == Suit::Spade {
                                    if self.spades_broken
                                        || !player_hand.iter().any(|c| c.suit != Suit::Spade)
                                    {
                                    } else {
                                        return Err(TransitionError::CardIncorrectSuit);
                                    }
                                }
                            }
                            if self.leading_suit != Some(card.suit)
                                && player_hand.iter().any(|x| Some(x.suit) == leading_suit)
                            {
                                return Err(TransitionError::CardIncorrectSuit);
                            }

                            if card.suit == Suit::Spade {
                                self.spades_broken = true;
                            }

                            let card_index = player_hand.iter().position(|x| x == &card).unwrap();
                            self.deck.push(player_hand.remove(card_index));
                        }

                        self.hands_played.last_mut().unwrap()[self.current_player_index] = Some(card);

                        if rotation_status == 3 {
                            let winner = self.scoring.trick(
                                self.current_player_index,
                                self.hands_played.last().unwrap(),
                            );
                            if self.scoring.is_over() {
                                self.state = State::GameCompleted;
                                return Ok(TransitionSuccess::GameOver);
                            }
                            if self.scoring.is_in_betting_stage() {
                                self.current_player_index = 0;
                                self.state = State::Betting((rotation_status + 1) % 4);
                                self.deal_cards(); // TODO this looks like an error
                            } else {
                                self.current_player_index = winner.unwrap();
                                self.state = State::Trick((rotation_status + 1) % 4); // TODO this should just be 0
                                self.hands_played.push([None; 4]);
                            }
                            Ok(TransitionSuccess::Trick)
                        } else {
                            self.current_player_index = (self.current_player_index + 1) % 4;
                            self.state = State::Trick((rotation_status + 1) % 4);
                            Ok(TransitionSuccess::PlayCard)
                        }
                    }
                }
            }
            GameAction::Start => {
                if self.state != State::GameNotStarted {
                    return Err(TransitionError::GameAlreadyStarted);
                }
                self.spades_broken = false;
                self.deal_cards();
                self.state = State::Betting(0);
                Ok(TransitionSuccess::Start)
            }
        }
    }

    fn deal_cards(&mut self) {
        //        cards::shuffle(&mut self.deck);
        let mut hands = cards::deal_four_players(&mut self.deck);

        self.player[0].hand = hands.pop().unwrap();
        self.player[1].hand = hands.pop().unwrap();
        self.player[2].hand = hands.pop().unwrap();
        self.player[3].hand = hands.pop().unwrap();

        self.player[0].hand.sort();
        self.player[1].hand.sort();
        self.player[2].hand.sort();
        self.player[3].hand.sort();
    }

    pub fn is_over(&self) -> bool {
        self.scoring.is_over()
    }

    pub fn team_a_round_score(&self) -> i32 {
        self.scoring.team[0].cumulative_points()
    }

    pub fn team_b_round_score(&self) -> i32 {
        self.scoring.team[1].cumulative_points()
    }
}
