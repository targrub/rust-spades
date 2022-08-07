/*
extern crate uuid;

use super::super::cards::{Card, Rank, Suit};
use super::super::game_state::State;
use super::super::result::{TransitionError, TransitionSuccess};
use super::super::{Game, GameAction, Bet};

#[allow(unused)]
#[test]
pub fn api_main_unit() {
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

    assert_eq!(
        g.execute_game_action(GameAction::Card(Card {
            suit: Suit::Heart,
            rank: Rank::Five
        })),
        Err(TransitionError::GameNotStarted)
    );
    assert_eq!(
        g.execute_game_action(GameAction::Bet(Bet::Amount(3))),
        Err(TransitionError::BetNotInBettingStage)
    );

    assert_eq!(g.execute_game_action(GameAction::Start), Ok(TransitionSuccess::Start));
    assert_eq!(
        g.execute_game_action(GameAction::Start),
        Err(TransitionError::GameAlreadyStarted)
    );

    let hand_a = vec![
        Card {
            suit: Suit::Club,
            rank: Rank::Five,
        },
        Card {
            suit: Suit::Club,
            rank: Rank::Ace,
        },
        Card {
            suit: Suit::Diamond,
            rank: Rank::Two,
        },
        Card {
            suit: Suit::Diamond,
            rank: Rank::Three,
        },
        Card {
            suit: Suit::Diamond,
            rank: Rank::King,
        },
        Card {
            suit: Suit::Diamond,
            rank: Rank::Ace,
        },
        Card {
            suit: Suit::Heart,
            rank: Rank::Five,
        },
        Card {
            suit: Suit::Heart,
            rank: Rank::Nine,
        },
        Card {
            suit: Suit::Heart,
            rank: Rank::Jack,
        },
        Card {
            suit: Suit::Heart,
            rank: Rank::King,
        },
        Card {
            suit: Suit::Heart,
            rank: Rank::Six,
        },
        Card {
            suit: Suit::Spade,
            rank: Rank::Six,
        },
        Card {
            suit: Suit::Spade,
            rank: Rank::Ace,
        },
    ];
    let hand_b = vec![
        Card {
            suit: Suit::Club,
            rank: Rank::Four,
        },
        Card {
            suit: Suit::Club,
            rank: Rank::Six,
        },
        Card {
            suit: Suit::Club,
            rank: Rank::Nine,
        },
        Card {
            suit: Suit::Club,
            rank: Rank::Jack,
        },
        Card {
            suit: Suit::Diamond,
            rank: Rank::Seven,
        },
        Card {
            suit: Suit::Heart,
            rank: Rank::Four,
        },
        Card {
            suit: Suit::Heart,
            rank: Rank::Eight,
        },
        Card {
            suit: Suit::Heart,
            rank: Rank::Queen,
        },
        Card {
            suit: Suit::Spade,
            rank: Rank::Two,
        },
        Card {
            suit: Suit::Spade,
            rank: Rank::Five,
        },
        Card {
            suit: Suit::Spade,
            rank: Rank::Eight,
        },
        Card {
            suit: Suit::Spade,
            rank: Rank::Ten,
        },
        Card {
            suit: Suit::Spade,
            rank: Rank::King,
        },
    ];
    let hand_c = vec![
        Card {
            suit: Suit::Club,
            rank: Rank::Two,
        },
        Card {
            suit: Suit::Club,
            rank: Rank::Seven,
        },
        Card {
            suit: Suit::Club,
            rank: Rank::Ten,
        },
        Card {
            suit: Suit::Diamond,
            rank: Rank::Five,
        },
        Card {
            suit: Suit::Diamond,
            rank: Rank::Eight,
        },
        Card {
            suit: Suit::Diamond,
            rank: Rank::Nine,
        },
        Card {
            suit: Suit::Diamond,
            rank: Rank::Jack,
        },
        Card {
            suit: Suit::Diamond,
            rank: Rank::Queen,
        },
        Card {
            suit: Suit::Heart,
            rank: Rank::Three,
        },
        Card {
            suit: Suit::Heart,
            rank: Rank::Seven,
        },
        Card {
            suit: Suit::Spade,
            rank: Rank::Nine,
        },
        Card {
            suit: Suit::Spade,
            rank: Rank::Jack,
        },
        Card {
            suit: Suit::Spade,
            rank: Rank::Queen,
        },
    ];
    let hand_d = vec![
        Card {
            suit: Suit::Club,
            rank: Rank::Three,
        },
        Card {
            suit: Suit::Club,
            rank: Rank::Eight,
        },
        Card {
            suit: Suit::Club,
            rank: Rank::Queen,
        },
        Card {
            suit: Suit::Club,
            rank: Rank::King,
        },
        Card {
            suit: Suit::Diamond,
            rank: Rank::Four,
        },
        Card {
            suit: Suit::Diamond,
            rank: Rank::Six,
        },
        Card {
            suit: Suit::Diamond,
            rank: Rank::Ten,
        },
        Card {
            suit: Suit::Heart,
            rank: Rank::Two,
        },
        Card {
            suit: Suit::Heart,
            rank: Rank::Ten,
        },
        Card {
            suit: Suit::Heart,
            rank: Rank::Ace,
        },
        Card {
            suit: Suit::Spade,
            rank: Rank::Three,
        },
        Card {
            suit: Suit::Spade,
            rank: Rank::Four,
        },
        Card {
            suit: Suit::Spade,
            rank: Rank::Seven,
        },
    ];

    g.player[0].hand = hand_a;
    g.player[1].hand = hand_b;
    g.player[2].hand = hand_c;
    g.player[3].hand = hand_d;
    assert_eq!(g.state, State::Betting(0));

    assert_eq!(
        g.execute_game_action(GameAction::Card(Card {
            suit: Suit::Heart,
            rank: Rank::Five
        })),
        Err(TransitionError::CardInBettingStage)
    );
    assert_eq!(
        g.execute_game_action(GameAction::Start),
        Err(TransitionError::GameAlreadyStarted)
    );
    assert_eq!(g.execute_game_action(GameAction::Bet(Bet::Amount(3))), Ok(TransitionSuccess::Bet));

    assert_eq!(
        g.execute_game_action(GameAction::Card(Card {
            suit: Suit::Heart,
            rank: Rank::Five
        })),
        Err(TransitionError::CardInBettingStage)
    );
    assert_eq!(g.execute_game_action(GameAction::Bet(Bet::Amount(3))), Ok(TransitionSuccess::Bet));

    assert_eq!(
        g.execute_game_action(GameAction::Card(Card {
            suit: Suit::Heart,
            rank: Rank::Five
        })),
        Err(TransitionError::CardInBettingStage)
    );
    assert_eq!(g.execute_game_action(GameAction::Bet(Bet::Amount(3))), Ok(TransitionSuccess::Bet));

    assert_eq!(
        g.execute_game_action(GameAction::Card(Card {
            suit: Suit::Heart,
            rank: Rank::Five
        })),
        Err(TransitionError::CardInBettingStage)
    );
    assert_eq!(
        g.execute_game_action(GameAction::Bet(Bet::Amount(3))),
        Ok(TransitionSuccess::BetComplete)
    );

    let mut trick_test_closure =
        |trick_number: usize, played_cards: &[Card; 4], team_a_won: usize| {
            assert_eq!(
                g.execute_game_action(GameAction::Card(played_cards[0].clone())),
                Ok(TransitionSuccess::PlayCard)
            );
            assert_eq!(
                g.execute_game_action(GameAction::Start),
                Err(TransitionError::GameAlreadyStarted)
            );
            assert_eq!(
                g.execute_game_action(GameAction::Bet(Bet::Amount(3))),
                Err(TransitionError::BetNotInBettingStage)
            );

            assert_eq!(
                g.execute_game_action(GameAction::Card(played_cards[1].clone())),
                Ok(TransitionSuccess::PlayCard)
            );
            assert_eq!(
                g.execute_game_action(GameAction::Start),
                Err(TransitionError::GameAlreadyStarted)
            );
            assert_eq!(
                g.execute_game_action(GameAction::Bet(Bet::Amount(3))),
                Err(TransitionError::BetNotInBettingStage)
            );

            assert_eq!(
                g.execute_game_action(GameAction::Card(played_cards[2].clone())),
                Ok(TransitionSuccess::PlayCard)
            );
            assert_eq!(
                g.execute_game_action(GameAction::Start),
                Err(TransitionError::GameAlreadyStarted)
            );
            assert_eq!(
                g.execute_game_action(GameAction::Bet(Bet::Amount(3))),
                Err(TransitionError::BetNotInBettingStage)
            );

            assert_eq!(
                g.execute_game_action(GameAction::Card(played_cards[3].clone())),
                Ok(TransitionSuccess::Trick)
            );
            assert_eq!(
                g.execute_game_action(GameAction::Start),
                Err(TransitionError::GameAlreadyStarted)
            );
            assert_eq!(
                g.execute_game_action(GameAction::Bet(Bet::Amount(3))),
                Err(TransitionError::BetNotInBettingStage)
            );

            assert_eq!(
                g.scoring.players[0].won_trick[trick_number] ||
                g.scoring.players[2].won_trick[trick_number],
                team_a_won == 1
            );
            assert_eq!(
                g.scoring.players[1].won_trick[trick_number] ||
                g.scoring.players[3].won_trick[trick_number],
                team_a_won == 0
            );
        };

    let pots = [
        [
            Card {
                suit: Suit::Club,
                rank: Rank::Ace,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Six,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Ten,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::King,
            },
        ],
        [
            Card {
                suit: Suit::Club,
                rank: Rank::Five,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Four,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Seven,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Queen,
            },
        ],
        [
            Card {
                suit: Suit::Club,
                rank: Rank::Eight,
            },
            Card {
                suit: Suit::Spade,
                rank: Rank::Six,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Nine,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Two,
            },
        ],
        [
            Card {
                suit: Suit::Club,
                rank: Rank::Ace,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Six,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Ten,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::King,
            },
        ],
        [
            Card {
                suit: Suit::Club,
                rank: Rank::Five,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Four,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Seven,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Queen,
            },
        ],
        [
            Card {
                suit: Suit::Club,
                rank: Rank::Eight,
            },
            Card {
                suit: Suit::Spade,
                rank: Rank::Six,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Nine,
            },
            Card {
                suit: Suit::Club,
                rank: Rank::Two,
            },
        ],
    ];
    let trick_winners = [0, 1, 0];
    // for t_n in 0..3 {
    //     trick_test_closure(t_n, &pots[t_n], trick_winners[t_n]);
    // }
}
*/
