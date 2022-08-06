extern crate rand;
extern crate spades;
extern crate uuid;

use rand::{thread_rng, Rng};
use spades::{Game, State};

fn main() {
    for _ in 0..1000 {
      let mut g = Game::new(
          uuid::Uuid::new_v4(),
          [
              uuid::Uuid::new_v4(),
              uuid::Uuid::new_v4(),
              uuid::Uuid::new_v4(),
              uuid::Uuid::new_v4(),
          ],
          500,
      );
      play_game(&mut g);
//            println!("winners: {:?}", g.get_winner_ids().unwrap());
//            println!("team a: bags: {} score: {}", g.get_team_a_bags().unwrap(), g.get_team_a_score().unwrap());
//            println!("team b: bags: {} score: {}", g.get_team_b_bags().unwrap(), g.get_team_b_score().unwrap());
  }
}

fn play_game(g: &mut Game) {
  let mut rng = thread_rng();
  loop {
    match g.get_state() {
      State::GameNotStarted => {
        g.start_game();
      },
      State::Betting(_player_index) => {
        if g.place_bet(3).is_err() {
          panic!("internal logic error");
        }
      },
      State::Trick(_player_index) => {
        let hand = g.get_current_hand().unwrap().clone();
        loop {
          if let Some(random_card) = rng.choose(hand.as_slice()) {
            // println!("player {} plays {}{}", playerindex, random_card.rank, random_card.suit);
            if g.play_card(*random_card).is_ok() {
              break;
            } else {
              // we're assuming the error was SpadesError::CardIncorrectSuit
              // println!("player {} tried to play {}{}, but it was the incorrect suit", playerindex, random_card.rank, random_card.suit);
              continue;
            } // randomly chosen card was wrong suit, choose another card
          } else {
            panic!("no valid card can be chosen");
          }
        }
      },
      State::GameCompleted => { return; }
    }
  }
}
