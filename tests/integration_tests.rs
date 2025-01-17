extern crate spades;

extern crate rand;

use rand::thread_rng;
use spades::{Bet, Card, Game, State, Suit};

#[test]
#[allow(unused)]
fn main() {
    let mut g = Game::default();

    g.start_game();
    while g.state() != State::GameCompleted {
        let mut rng = thread_rng();

        if let State::Trick(_playerindex) = g.state() {
            assert!(g.current_hand().is_ok());
            let mut hand = g.current_hand().ok().unwrap().clone();

            let leading_suit_opt = g.leading_suit().unwrap();
            let x = get_valid_card_index(leading_suit_opt, &hand);

            if let Some(response) = g.can_play_card(hand[x].clone()) {
                // first choice failed, so we'll try each card until one works
                let mut worked = false;
                let mut y = (x + 1) % hand.len();
                for y in (x + 1)..hand.len() {
                    if let Some(response) = g.can_play_card(hand[y].clone()) {
                    } else {
                        g.play_card(hand[y].clone());
                        worked = true;
                        break;
                    }
                }
                if !worked {
                    for y in 0..hand.len() {
                        if let Some(response) = g.can_play_card(hand[y].clone()) {
                        } else {
                            g.play_card(hand[y].clone());
                            break;
                        }
                    }
                }
            } else {
                // we're good
                g.play_card(hand[x].clone());
            }
        } else {
            g.place_bet(Bet::Amount(3));
        }
    }
    assert_eq!(g.state(), State::GameCompleted);
}

pub fn get_valid_card_index(leading_suit: Option<Suit>, hand: &Vec<Card>) -> usize {
    if hand.iter().any(|ref x| Some(x.suit) == leading_suit) {
        hand.iter()
            .position(|ref x| Some(x.suit) == leading_suit)
            .unwrap()
    } else {
        if let Some(card_index) = hand.iter().position(|c| c.suit == Suit::Spades) {
            card_index
        } else {
            if let Some(card_index) = hand
                .iter()
                .position(|c| c.suit != Suit::Spades && Some(c.suit) != leading_suit)
            {
                card_index
            } else {
                0
            }
        }
    }
}
