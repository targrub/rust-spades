//! This crate provides an implementation of the four person card game, [Spades](https://www.pagat.com/auctionwhist/spades.html).
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
//!
//!
//! The sequence of a round of the game is expected to go as follows:
//! Start -> Bet * 4 -> Card * 4 * 13 -> [End of a round] -> Bet * 4 -> Card * 4 * 13 -> Bet * 4 -> ...
//!
//! The game is in` State` `GameNotStarted` until it is started via a `start_game()` call.
//! That moves it to `State` `Betting(player_number)`.  Once all 4 players have bet, the game mvoes to
//! `State` `Trick(player_number)`.  After 13 tricks of cards played by each of the 4 players, the round is over.
//! The game `State` will move either back to the `Betting` state for a new round of the game, or to `GameCompleted`
//! if one team has scored enough cumulative points to have won the game (at least as many as the `max_points`
//! parameter given to `Game::new()`).
//!

extern crate uuid;

mod cards;
mod game_state;
mod result;
mod scoring;

#[cfg(test)]
mod tests;

pub use cards::{Card, Rank, Suit};
pub use game_state::State;
pub use result::SpadesError;
pub use scoring::Bet;

/// If a bet is made successfully, this lets one distinguish whether that bet ends the round of betting.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum BetResult {
    /// Bet was made successfully.
    #[default]
    MadeBet,
    /// This bet completed the betting stage.
    CompletedBetting,
}

/// If a card is played successfully, this lets one distinguish whether that card results in the completion
/// of a trick, or even the entire game.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum PlayCardResult {
    /// The card was played successfully.
    #[default]
    CardPlayed,
    /// The playing of the card completed a trick.
    TrickCompleted,
    /// The playing of the card completed the game.
    GameCompleted,
}

use uuid::Uuid;

use cards::{deal_four_players, new_deck};
use scoring::Scoring;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Player {
    id: Uuid,
    seen_hand: bool,
    hand: Vec<Card>,
}

impl Player {
    fn new(id: Uuid) -> Player {
        Player {
            id,
            seen_hand: false,
            hand: vec![],
        }
    }
}

/// Primary game state. Internally manages player rotation, scoring, and cards.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Game {
    id: Uuid,
    state: State,
    scoring: Scoring,
    current_player_index: usize,
    deck: Vec<Card>,
    current_trick: Vec<Card>,
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
            scoring: Scoring::default(),
            current_player_index: 0,
            deck: new_deck(),
            leading_suit: None,
            spades_broken: false,
            current_trick: Vec::new(),
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
            scoring: Scoring::new(max_points),
            current_trick: Vec::new(),
            bets_placed: [Bet::Amount(0); 4],
            deck: new_deck(),
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

    /// The uuid of the game itself
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// See [`State`](enum.State.html)
    pub fn state(&self) -> State {
        self.state
    }

    /// Score for Team A (players 0 and 2) for the round just finished, valid at the end of each round.
    pub fn team_a_individual_round_score(&self) -> Result<i32, SpadesError> {
        match self.state {
            State::GameNotStarted => Err(SpadesError::GameNotStarted),
            _ => Ok(self.scoring.team[0].game_points()),
        }
    }

    /// Score for Team B (players 1 and 3) for the round just finished, valid at the end of each round.
    pub fn team_b_individual_round_score(&self) -> Result<i32, SpadesError> {
        match self.state {
            State::GameNotStarted => Err(SpadesError::GameNotStarted),
            _ => Ok(self.scoring.team[1].game_points()),
        }
    }

    /// Score for Team A (players 0 and 2) so far in the game, valid at the end of each round.
    pub fn team_a_all_rounds_score(&self) -> Result<i32, SpadesError> {
        match self.state {
            State::GameNotStarted => Err(SpadesError::GameNotStarted),
            _ => Ok(self.scoring.team[0].cumulative_points()),
        }
    }

    /// Score for Team B (players 1 and 3) so far in the game, valid at the end of each round.
    pub fn team_b_all_rounds_score(&self) -> Result<i32, SpadesError> {
        match self.state {
            State::GameNotStarted => Err(SpadesError::GameNotStarted),
            _ => Ok(self.scoring.team[1].cumulative_points()),
        }
    }

    /// Number of tricks taken by Team A (players 0 and 2) for the round just completed.
    pub fn team_a_tricks(&self) -> Result<u8, SpadesError> {
        match self.state {
            State::GameNotStarted => Err(SpadesError::GameNotStarted),
            _ => Ok(self.scoring.team[0].tricks_won()),
        }
    }

    /// Number of tricks taken by Team B (players 1 and 3) for the round just completed.
    pub fn team_b_tricks(&self) -> Result<u8, SpadesError> {
        match self.state {
            State::GameNotStarted => Err(SpadesError::GameNotStarted),
            _ => Ok(self.scoring.team[1].tricks_won()),
        }
    }

    /// Number of bags (overtricks) taken by Team A (players 0 and 2) for the round just completed.
    pub fn team_a_individual_round_bags(&self) -> Result<u8, SpadesError> {
        match self.state {
            State::GameNotStarted => Err(SpadesError::GameNotStarted),
            _ => Ok(self.scoring.team[0].game_bags()),
        }
    }

    /// Number of bags (overtricks) taken by Team B (players 1 and 3) for the round just completed.
    pub fn team_b_individual_round_bags(&self) -> Result<u8, SpadesError> {
        match self.state {
            State::GameNotStarted => Err(SpadesError::GameNotStarted),
            _ => Ok(self.scoring.team[1].game_bags()),
        }
    }

    /// Number of bags (overtricks) taken by Team A (players 0 and 2) for all rounds completed.
    /// Decremented by 10 when over 10, decreasing the overall score for this team.
    pub fn team_a_all_rounds_bags(&self) -> Result<u8, SpadesError> {
        match self.state {
            State::GameNotStarted => Err(SpadesError::GameNotStarted),
            _ => Ok(self.scoring.team[0].cumulative_bags()),
        }
    }

    /// Number of bags (overtricks) taken by Team B (players 1 and 3) for all rounds completed.
    /// Decremented by 10 when over 10, decreasing the overall score for this team.
    pub fn team_b_all_rounds_bags(&self) -> Result<u8, SpadesError> {
        match self.state {
            State::GameNotStarted => Err(SpadesError::GameNotStarted),
            _ => Ok(self.scoring.team[1].cumulative_bags()),
        }
    }

    /// Obtain the uuid of the player expected to take the next game action.
    /// Returns `SpadesError` when the current game is not in the Betting or Trick stages.
    pub fn current_player_id(&self) -> Result<&Uuid, SpadesError> {
        match (&self.state, self.current_player_index) {
            (State::GameNotStarted, _) => Err(SpadesError::GameNotStarted),
            (State::GameCompleted, _) => Err(SpadesError::GameCompleted),
            (State::Betting(_), p) | (State::Trick(_), p) => Ok(&self.player[p].id),
        }
    }

    /// Obtain the set of cards in the hand of the player with the matching uuid.
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

    /// Obtain the set of cards in the hand of the player expected to take the next game action.
    /// Once this is called for a player, they may not make a blind nil bid for that round.
    pub fn current_hand(&mut self) -> Result<&Vec<Card>, SpadesError> {
        match (&self.state, self.current_player_index) {
            (State::GameNotStarted, _) => Err(SpadesError::GameNotStarted),
            (State::GameCompleted, _) => Err(SpadesError::GameCompleted),
            (State::Betting(_), p) | (State::Trick(_), p) => {
                self.player[p].seen_hand = true;
                Ok(&self.player[p].hand)
            }
        }
    }

    /// The suit led for the current trick.
    pub fn leading_suit(&self) -> Result<Option<Suit>, SpadesError> {
        match &self.state {
            State::GameNotStarted => Err(SpadesError::GameNotStarted),
            State::GameCompleted => Err(SpadesError::GameCompleted),
            State::Trick(_) => Ok(self.leading_suit),
            _ => Err(SpadesError::InternalError),
        }
    }

    // Obtain the uuids of the players on the team that won this game.
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

    // Obtain the bets that have been placed by each player for the current round.
    pub fn bets_placed(&self) -> Result<[Bet; 4], SpadesError> {
        Ok(self.bets_placed)
    }

    /// Use this method to check whether the game is expecting start_game to be called next.
    ///
    /// If you want to check for errors:
    ///
    /// let mut g = Game::default();
    /// if let Some(why_not) = g.can_start_game() {
    ///    // library user error
    /// } else {
    ///  g.start_game();
    /// }
    ///
    /// don't check for errors
    /// g.start_game();
    pub fn can_start_game(&self) -> Option<SpadesError> {
        if self.state == State::GameNotStarted {
            None
        } else {
            Some(SpadesError::ImproperGameStage)
        }
    }

    /// Start the game, moving it into the betting stage.
    pub fn start_game(&mut self) {
        if let Some(_err) = self.can_start_game() {
            // don't do anything if can't start game
        } else {
            self.execute_game_start();
        }
    }

    /// Use this method to know whether it is valid to make this bet.
    ///
    /// If you want to check for errors:
    /// let mut g = Game::default();
    /// let bet = Bet::Amount(5);
    /// if let Some(why_not) = g.can_place_bet(bet) {
    ///    // library user error why_not of type SpadesError
    /// } else {
    ///  if let Some(bet_result) = g.place_bet(bet) {
    ///    // bet_result either BetResult::SuccessfulBet or BetResult::SuccessfulBetCompletedBetting
    ///  }
    /// }
    /// If you don't want check for errors:
    /// let bet: Bet = Bet::Amount(5);
    /// g.place_bet(bet);
    pub fn can_place_bet(&self, bet: Bet) -> Option<SpadesError> {
        match self.state {
            State::GameNotStarted => Some(SpadesError::GameNotStarted),
            State::Trick(_) => Some(SpadesError::ImproperGameStage),
            State::GameCompleted => Some(SpadesError::GameCompleted),
            State::Betting(_rotation_status) => {
                if bet == Bet::BlindNil && self.player[self.current_player_index].seen_hand {
                    Some(SpadesError::BetImproperSeenHand)
                } else {
                    None
                }
            }
        }
    }

    /// Make this bet for the current player.
    pub fn place_bet(&mut self, bet: Bet) -> Option<BetResult> {
        if let Some(_err) = self.can_place_bet(bet) {
            // don't do anything if can't make the bet
            None
        } else if let State::Betting(rotation_status) = self.state {
            let bet_result = self.execute_bet(rotation_status, bet);
            Some(bet_result)
        } else {
            None
        }
    }

    /// A method to determine whether a card may be played by the current player.
    /// If it would not be possible, the reason why not will be returned in Some(SpadesError).
    pub fn can_play_card(&self, card: Card) -> Option<SpadesError> {
        match self.state {
            State::GameNotStarted => Some(SpadesError::GameNotStarted),
            State::GameCompleted => Some(SpadesError::GameCompleted),
            State::Betting(_rotation_status) => Some(SpadesError::ImproperGameStage),
            State::Trick(rotation_status) => {
                let player_hand = &self.player[self.current_player_index].hand;
                self.can_play_card_from_hand(rotation_status, card, player_hand)
            }
        }
    }

    /// Play this card for the current player.
    /// If the card is successfully played, it will return Some(PlayCardResult);
    /// otherwise it will return None.
    pub fn play_card(&mut self, card: Card) -> Option<PlayCardResult> {
        if let Some(_err) = self.can_play_card(card) {
            // don't do anything if can't play this card
            None
        } else if let State::Trick(rotation_status) = self.state {
            self.leading_suit = Some(card.suit);
            let card_index = self.player[self.current_player_index]
                .hand
                .iter()
                .position(|x| x == &card)
                .unwrap();
            self.deck.push(
                self.player[self.current_player_index]
                    .hand
                    .remove(card_index),
            );

            let card_result = self.execute_play_card(rotation_status, card);
            Some(card_result)
        } else {
            None
        }
    }

    fn execute_game_start(&mut self) {
        self.spades_broken = false;
        self.deal_cards();
        self.state = State::Betting(0);
    }

    fn execute_bet(&mut self, rotation_status: usize, bet: Bet) -> BetResult {
        self.scoring.add_bet(self.current_player_index, bet);
        if rotation_status == 3 {
            self.scoring.betting_over();
            self.state = State::Trick((rotation_status + 1) % 4);
            self.current_player_index = 0;
            BetResult::CompletedBetting
        } else {
            self.current_player_index = (self.current_player_index + 1) % 4;
            self.state = State::Betting((rotation_status + 1) % 4);
            BetResult::MadeBet
        }
    }

    fn execute_play_card(&mut self, rotation_status: usize, card: Card) -> PlayCardResult {
        if card.suit == Suit::Spade {
            self.spades_broken = true;
        }

        self.current_trick.push(card);

        if rotation_status == 3 {
            let winner = self
                .scoring
                .trick(self.current_player_index, &self.current_trick); // NOTE: Is this the right parameter value?  It ought to be whoever led the trick.
            self.current_trick.clear();
            if self.scoring.is_over() {
                self.state = State::GameCompleted;
                return PlayCardResult::GameCompleted;
            }
            if self.scoring.is_in_betting_stage() {
                self.current_player_index = 0;
                self.state = State::Betting((rotation_status + 1) % 4);
                self.deal_cards(); // NOTE: The deal should happen when move from Start to Betting
            } else {
                self.current_player_index = winner; // the trick winner will lead on the next trick
                self.state = State::Trick((rotation_status + 1) % 4); // NOTE: Why not current_player_index?
            }
            PlayCardResult::TrickCompleted
        } else {
            self.current_player_index = (self.current_player_index + 1) % 4;
            self.state = State::Trick((rotation_status + 1) % 4); // NOTE: Why not current_player_index?
            PlayCardResult::CardPlayed
        }
    }

    fn can_play_card_from_hand(
        &self,
        rotation_status: usize,
        card: Card,
        hand: &[Card],
    ) -> Option<SpadesError> {
        if !hand.contains(&card) {
            return Some(SpadesError::CardNotInHand);
        }
        let leading_suit = self.leading_suit;
        if rotation_status == 0 {
            // to lead spades, spades must be broken OR only have spades in this hand
            if card.suit == Suit::Spade {
                if self.spades_broken || !hand.iter().any(|c| c.suit != Suit::Spade) {
                } else {
                    return Some(SpadesError::CardIncorrectSuit);
                }
            }
        }
        if self.leading_suit != Some(card.suit) && hand.iter().any(|x| Some(x.suit) == leading_suit)
        {
            return Some(SpadesError::CardIncorrectSuit);
        }
        None
    }

    fn deal_cards(&mut self) {
        //        cards::shuffle(&mut self.deck);
        let mut hands = deal_four_players(&mut self.deck);

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
}

#[cfg(test)]
mod game_tests {

    #![allow(unused_variables)]

    use Bet;
    use Game;
    use SpadesError;
    use State;

    use crate::{BetResult, PlayCardResult};

    #[test]
    fn test_create_game() {
        let game_uuid = uuid::Uuid::new(uuid::UuidVersion::Random).unwrap();
        let p1_uuid = uuid::Uuid::new(uuid::UuidVersion::Random).unwrap();
        let p2_uuid = uuid::Uuid::new(uuid::UuidVersion::Random).unwrap();
        let p3_uuid = uuid::Uuid::new(uuid::UuidVersion::Random).unwrap();
        let p4_uuid = uuid::Uuid::new(uuid::UuidVersion::Random).unwrap();
        let player_uuids = [p1_uuid, p2_uuid, p3_uuid, p4_uuid];
        let max_points: i32 = -1;

        let g = Game::new(game_uuid, player_uuids, max_points);
        let cpi = g.current_player_index;
        assert_eq!(0, cpi);
        let curr_trick = g.current_trick;
        assert!(curr_trick.is_empty());
        let deck = g.deck;
        assert_eq!(52, deck.len());
        let gameid = g.id;
        assert_eq!(game_uuid, gameid);
        let leading_suit = g.leading_suit;
        assert_eq!(None, leading_suit);
        let players = g.player;
        assert_eq!(p1_uuid, players[0].id);
        assert_eq!(p2_uuid, players[1].id);
        assert_eq!(p3_uuid, players[2].id);
        assert_eq!(p4_uuid, players[3].id);
        let b = g.scoring;
        let spades_broken = g.spades_broken;
        assert_eq!(false, spades_broken);
        let gamestate = g.state;
        assert_eq!(State::GameNotStarted, gamestate);
    }

    #[test]
    fn test_default_game() {
        let g = Game::default();
        let cpi = g.current_player_index;
        assert_eq!(0, cpi);
        let curr_trick = g.current_trick;
        assert!(curr_trick.is_empty());
        let deck = g.deck;
        assert_eq!(52, deck.len());
        let leading_suit = g.leading_suit;
        assert_eq!(None, leading_suit);
        let players = g.player;
        assert!(players[0].hand.is_empty());
        let b = g.scoring;
        let spades_broken = g.spades_broken;
        assert_eq!(false, spades_broken);
        let gamestate = g.state;
        assert_eq!(State::GameNotStarted, gamestate);
    }

    #[test]
    fn test_queries_when_gamenotstarted() {
        let g = Game::default();
        assert_eq!(
            Err(SpadesError::GameNotStarted),
            g.team_a_individual_round_bags()
        );
        assert_eq!(
            Err(SpadesError::GameNotStarted),
            g.team_a_individual_round_score()
        );
        assert_eq!(Err(SpadesError::GameNotStarted), g.team_a_all_rounds_bags());
        assert_eq!(
            Err(SpadesError::GameNotStarted),
            g.team_a_all_rounds_score()
        );
        assert_eq!(Err(SpadesError::GameNotStarted), g.team_a_tricks());
        assert_eq!(
            Err(SpadesError::GameNotStarted),
            g.team_b_individual_round_bags()
        );
        assert_eq!(
            Err(SpadesError::GameNotStarted),
            g.team_b_individual_round_score()
        );
        assert_eq!(Err(SpadesError::GameNotStarted), g.team_b_all_rounds_bags());
        assert_eq!(
            Err(SpadesError::GameNotStarted),
            g.team_b_all_rounds_score()
        );
        assert_eq!(Err(SpadesError::GameNotStarted), g.team_b_tricks());
    }

    #[test]
    fn test_current_player_id_and_blind_nil_bets() {
        let game_uuid = uuid::Uuid::new(uuid::UuidVersion::Random).unwrap();
        let p1_uuid = uuid::Uuid::new(uuid::UuidVersion::Random).unwrap();
        let p2_uuid = uuid::Uuid::new(uuid::UuidVersion::Random).unwrap();
        let p3_uuid = uuid::Uuid::new(uuid::UuidVersion::Random).unwrap();
        let p4_uuid = uuid::Uuid::new(uuid::UuidVersion::Random).unwrap();
        let player_uuids = [p1_uuid, p2_uuid, p3_uuid, p4_uuid];
        let mut g = Game::new(game_uuid, player_uuids, 500);
        let mut cpi_response = g.current_player_id();
        assert_eq!(Err(SpadesError::GameNotStarted), cpi_response);
        g.start_game();
        cpi_response = g.current_player_id();
        assert_eq!(Ok(&p1_uuid), cpi_response);
        let look_at_hand_response = g.current_hand();
        assert_eq!(true, look_at_hand_response.is_ok());
        assert_eq!(13, look_at_hand_response.unwrap().len());
        let mut can_bet_response = g.can_place_bet(Bet::BlindNil);
        assert_eq!(Some(SpadesError::BetImproperSeenHand), can_bet_response);
        can_bet_response = g.can_place_bet(Bet::Nil);
        assert_eq!(None, can_bet_response);
        let mut place_bet_response = g.place_bet(Bet::Nil);
        assert_eq!(Some(BetResult::MadeBet), place_bet_response);
        cpi_response = g.current_player_id();
        assert_eq!(Ok(&p2_uuid), cpi_response);
        place_bet_response = g.place_bet(Bet::Amount(3));
        assert_eq!(Some(BetResult::MadeBet), place_bet_response);
        cpi_response = g.current_player_id();
        assert_eq!(Ok(&p3_uuid), cpi_response);
        place_bet_response = g.place_bet(Bet::BlindNil);
        assert_eq!(Some(BetResult::MadeBet), place_bet_response);
        cpi_response = g.current_player_id();
        assert_eq!(Ok(&p4_uuid), cpi_response);
        place_bet_response = g.place_bet(Bet::Amount(3));
        assert_eq!(Some(BetResult::CompletedBetting), place_bet_response);
        cpi_response = g.current_player_id();
        assert_eq!(Ok(&p1_uuid), cpi_response);
        let card_to_play = g.current_hand().unwrap()[0];
        let play_card_action_response = g.play_card(card_to_play);
        assert_eq!(Some(PlayCardResult::CardPlayed), play_card_action_response);
        cpi_response = g.current_player_id();
        assert_eq!(Ok(&p2_uuid), cpi_response);
    }

    #[test]
    fn test_hand_from_player_id() {
        let game_uuid = uuid::Uuid::new(uuid::UuidVersion::Random).unwrap();
        let p1_uuid = uuid::Uuid::new(uuid::UuidVersion::Random).unwrap();
        let p2_uuid = uuid::Uuid::new(uuid::UuidVersion::Random).unwrap();
        let p3_uuid = uuid::Uuid::new(uuid::UuidVersion::Random).unwrap();
        let p4_uuid = uuid::Uuid::new(uuid::UuidVersion::Random).unwrap();
        let unknown_uuid = uuid::Uuid::new(uuid::UuidVersion::Random).unwrap();
        let player_uuids = [p1_uuid, p2_uuid, p3_uuid, p4_uuid];
        let mut g = Game::new(game_uuid, player_uuids, 500);
        g.start_game();
        let p1_hand_result = g.hand_from_player_id(p1_uuid);
        if let Ok(p1_hand) = p1_hand_result {
            assert_eq!(13, p1_hand.len());
        } else {
            assert!(false); // p1 is a valid player, so should not error
        }
        let unknown_hand_result = g.hand_from_player_id(unknown_uuid);
        assert_eq!(Err(SpadesError::InvalidUuid), unknown_hand_result);
        if let Ok(p1_hand) = p1_hand_result {
            assert_eq!(13, p1_hand.len());
        } else {
        }
    }
}
