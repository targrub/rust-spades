extern crate rand;
extern crate spades;
extern crate uuid;

use rand::{thread_rng, Rng};
use spades::{Game, GameTransition, State, TransitionError, TransitionSuccess};

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
    match g.play(GameTransition::Start) {
        Ok(_) => {
            while g.get_state() != State::GameCompleted {
                let mut rng = thread_rng();
                if let State::Trick(playerindex) = g.get_state() {
                    assert!(g.get_current_hand().is_ok());
                    let hand = g.get_current_hand().unwrap().clone();

                    loop {
                        if let Some(random_card) = rng.choose(hand.as_slice()) {
                            let state = g.get_state();
                            match state {
                                State::Trick(x) => {
//                                    println!(
//                                        "player {} plays {}{}",
//                                        x, random_card.rank, random_card.suit
//                                    );
                                }
                                _ => {
                                    panic!("should be playing trick")
                                }
                            }
                            match g.play(GameTransition::Card(*random_card)) {
                                Ok(TransitionSuccess::PlayCard) => {
                                    break;
                                }
                                Ok(TransitionSuccess::Trick) => {
                                    //println!("trick over");
                                    break;
                                }
                                Ok(TransitionSuccess::GameOver) => {
                                    //println!("game over");
                                    break;
                                }
                                Ok(_) => {
                                    println!("unexpected result of card being played")
                                }
                                Err(TransitionError::CardIncorrectSuit) => {
//                                    println!("player {} tried to play {}{}, but it was the incorrect suit", playerindex, random_card.rank, random_card.suit);
                                    continue;
                                } // randomly chosen card was wrong suit, choose another card
                                Err(TransitionError::CardNotInHand) => {
                                    panic!("chose card that wasn't in the hand");
                                }
                                Err(_) => {
                                    panic!("internal logic error");
                                }
                            }
                        } else {
                            panic!("no valid card can be chosen");
                        }
                    }
                } else {
                    match g.play(GameTransition::Bet(3)) {
                        Ok(_) => {}
                        Err(_) => {
                            panic!("internal logic error");
                        }
                    }
                }
            }
            assert_eq!(g.get_state(), State::GameCompleted);
        }
        Err(_) => {
            panic!("unable to start game");
        }
    }
}
