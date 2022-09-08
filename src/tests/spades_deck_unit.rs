use super::super::cards;
use super::super::cards::{deal_four_players, get_trick_winner, Card, Rank, Suit};
#[test]
fn new_deck() {
    let x = cards::new_deck();
    assert_eq!(x.len(), 52);
}

#[test]
fn deal_deck_four_players() {
    let mut x = cards::new_deck();

    let hands = deal_four_players(&mut x);
    assert_eq!(hands[0].len(), 13);
    assert_eq!(hands[1].len(), 13);
    assert_eq!(hands[2].len(), 13);
    assert_eq!(hands[3].len(), 13);
}

#[test]
fn trick_winner_same_suit() {
    let a = Card {
        suit: Suit::Clubs,
        rank: Rank::Two,
    };
    let b = Card {
        suit: Suit::Clubs,
        rank: Rank::Ace,
    };
    let c = Card {
        suit: Suit::Clubs,
        rank: Rank::King,
    };
    let d = Card {
        suit: Suit::Clubs,
        rank: Rank::Nine,
    };

    let trick = vec![a, b, c, d];
    assert_eq!(1, get_trick_winner(0, &trick));
    assert_eq!(2, get_trick_winner(1, &trick));
    assert_eq!(3, get_trick_winner(2, &trick));
    assert_eq!(0, get_trick_winner(3, &trick));
}

#[test]
fn trick_winner_no_spades() {
    let a = Card {
        suit: Suit::Diamonds,
        rank: Rank::Two,
    };
    let b = Card {
        suit: Suit::Hearts,
        rank: Rank::Ace,
    };
    let c = Card {
        suit: Suit::Hearts,
        rank: Rank::King,
    };
    let d = Card {
        suit: Suit::Diamonds,
        rank: Rank::Nine,
    };

    let trick = vec![a, b, c, d];

    assert_eq!(3, get_trick_winner(0, &trick));
    assert_eq!(0, get_trick_winner(1, &trick));
    assert_eq!(1, get_trick_winner(2, &trick));
    assert_eq!(2, get_trick_winner(3, &trick));
}

#[test]
fn trick_winner_spades() {
    let a = Card {
        suit: Suit::Diamonds,
        rank: Rank::Two,
    };
    let b = Card {
        suit: Suit::Hearts,
        rank: Rank::Ace,
    };
    let c = Card {
        suit: Suit::Spades,
        rank: Rank::Two,
    };
    let d = Card {
        suit: Suit::Diamonds,
        rank: Rank::Nine,
    };

    let trick = vec![a, b, c, d];

    assert_eq!(2, get_trick_winner(0, &trick));
    assert_eq!(3, get_trick_winner(1, &trick));
    assert_eq!(0, get_trick_winner(2, &trick));
    assert_eq!(1, get_trick_winner(3, &trick));
}
