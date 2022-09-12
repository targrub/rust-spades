extern crate rand;
extern crate spades;

use rand::{thread_rng, Rng};
use spades::{Bet, Game, State, Uid};

fn main() {
    let game_id = Uid(12345);
    let player_ids = [Uid(1), Uid(2), Uid(3), Uid(4)];
    for _r in 0..1000 {
        // rounds
        //println!("round {}", r + 1);
        let mut g = Game::assign_players(game_id, player_ids);
        play_complete_round(&mut g);
        /*
        println!("winners of final game: {:?}", g.get_winner_ids().unwrap());
        println!(
            "team a: tricks: {}  bags: {}  game score: {}  cumulative score: {}",
            g.get_team_a_tricks().unwrap(),
            g.get_team_a_bags().unwrap(),
            g.get_team_a_game_score().unwrap(),
            g.get_team_a_round_score()
        );
        println!(
            "team b: tricks: {}  bags: {}  game score: {}  cumulative score: {}",
            g.get_team_b_tricks().unwrap(),
            g.get_team_b_bags().unwrap(),
            g.get_team_b_game_score().unwrap(),
            g.get_team_b_round_score()
        );
        */
    }
}

fn play_complete_round(g: &mut Game) {
    let mut rng = thread_rng();
    loop {
        match g.state() {
            State::GameNotStarted => {
                g.start_game();
            }
            State::Betting(_player_index) => {
                g.place_bet(Bet::Amount(3));
            }
            State::Trick(_player_index) => {
                let hand = g.current_hand().unwrap().clone();
                let mut times_through = 0;
                let mut last_choice = None;
                let mut last_err = None;
                loop {
                    times_through += 1;
                    if times_through > 1000 {
                        println!("{:?}", g);
                        println!("{:?}", hand);
                        println!("{:?}", last_choice);
                        println!("{:?}", last_err);
                        panic!("should have something to play");
                    }
                    if let Some(random_card) = rng.choose(hand.as_slice()) {
                        // println!("player {} plays {}{}", playerindex, random_card.rank, random_card.suit);
                        last_choice = Some(*random_card);
                        if let Some(err) = g.can_play_card(*random_card) {
                            // we're assuming the error was SpadesError::CardIncorrectSuit
                            // println!("player {} tried to play {}{}, but it was the incorrect suit", playerindex, random_card.rank, random_card.suit);
                            last_err = Some(err);
                            continue;
                        } else {
                            g.play_card(*random_card);
                            break;
                        }
                    } else {
                        panic!("no valid card can be chosen");
                    }
                }
            }
            State::GameCompleted => {
                return;
            }
        }
    }
}
