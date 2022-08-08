#![allow(unused)]

extern crate rand;

use self::rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::fmt::{self, Display};

#[derive(Default, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Debug, Hash)]
pub enum Suit {
    #[default]
    Blank = 0,
    Club = 1,
    Diamond = 2,
    Heart = 3,
    Spade = 4,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suit::Blank => write!(f, " "),
            Suit::Club => write!(f, "\u{2667}"),
            Suit::Diamond => write!(f, "\u{2662}"),
            Suit::Heart => write!(f, "\u{2661}"),
            Suit::Spade => write!(f, "\u{2664}"),
        }
    }
}

impl From<u8> for Suit {
    fn from(f: u8) -> Self {
        match f {
            0 => Suit::Blank,
            1 => Suit::Club,
            2 => Suit::Diamond,
            3 => Suit::Heart,
            4 => Suit::Spade,
            _ => Suit::Blank,
        }
    }
}

#[test]
fn test_from_u8_to_suit() {
    let s: Suit = 3u8.into();
    assert_eq!(Suit::Heart, s);
    assert_eq!(Suit::Blank, 0u8.into());
    assert_eq!(Suit::Club, 1u8.into());
    assert_eq!(Suit::Diamond, 2u8.into());
    assert_eq!(Suit::Heart, 3u8.into());
    assert_eq!(Suit::Spade, 4u8.into());
    assert_eq!(Suit::Blank, 5u8.into());
    assert_eq!(Suit::Blank, 14u8.into());
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Debug, Hash)]
pub enum Rank {
    #[default]
    Blank = 0,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl From<u8> for Rank {
    fn from(u: u8) -> Self {
        match u {
            2 => Rank::Two,
            3 => Rank::Three,
            4 => Rank::Four,
            5 => Rank::Five,
            6 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            9 => Rank::Nine,
            10 => Rank::Ten,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            14 => Rank::Ace,
            _ => Rank::Blank,
        }
    }
}

#[test]
fn test_from_u8_to_rank() {
    let r: Rank = 3u8.into();
    assert_eq!(Rank::Three, r);
    assert_eq!(Rank::Ace, 14u8.into());
    assert_eq!(Rank::Ten, 10u8.into());
    assert_eq!(Rank::Two, 2u8.into());
    assert_eq!(Rank::Blank, 0u8.into());
    assert_eq!(Rank::Blank, 1u8.into());
    assert_eq!(Rank::Blank, 15u8.into());
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rank::Blank => write!(f, " "),
            Rank::Two => write!(f, "2"),
            Rank::Three => write!(f, "3"),
            Rank::Four => write!(f, "4"),
            Rank::Five => write!(f, "5"),
            Rank::Six => write!(f, "6"),
            Rank::Seven => write!(f, "7"),
            Rank::Eight => write!(f, "8"),
            Rank::Nine => write!(f, "9"),
            Rank::Ten => write!(f, "10"),
            Rank::Jack => write!(f, "J"),
            Rank::Queen => write!(f, "Q"),
            Rank::King => write!(f, "K"),
            Rank::Ace => write!(f, "A"),
        }
    }
}

/// Intuitive card struct. Comparisons are made according to alphabetical order, ascending.
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    fn new(suit: Suit, rank: Rank) -> Card {
        Card { suit, rank }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?}", self.rank, self.suit)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Card) -> Ordering {
        ((self.suit as u64) * 15 + (self.rank as u64))
            .cmp(&(((other.suit as u64) * 15) + (other.rank as u64)))
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        Some(
            ((self.suit as u64) * 15 + (self.rank as u64))
                .cmp(&(((other.suit as u64) * 15) + (other.rank as u64))),
        )
    }
}

/// Given four cards and a starting card, returns the winner of a trick.
///
/// The rules used to determine the winner of a trick are as follows:
/// * Spades trump all other suits
/// * The suit the first player (given by index) plays sets the suit of the trick
/// * The highest ranking spades card or card of suit of first player's card wins the trick.
pub fn get_trick_winner(leading_player_index: usize, others: &[Option<Card>; 4]) -> Option<usize> {
    let mut winning_index = leading_player_index;
    if let Some(mut best_card) = others[leading_player_index] {
        for (i, other) in others.iter().enumerate() {
            if let Some(card) = *other {
                if card.suit == best_card.suit {
                    if card.rank as u8 > best_card.rank as u8 {
                        best_card = card;
                        winning_index = i;
                    }
                } else if card.suit == Suit::Spade {
                    best_card = card;
                    winning_index = i;
                }
            }
        }
        Some(winning_index)
    } else {
        None
    }
}

/// Returns a shuffled deck of [`deck::Card`](struct.Card.html)'s, with 52 elements.
pub fn new_deck() -> Vec<Card> {
    let ranks: Vec<Rank> = vec![
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
        Rank::Ace,
    ];
    let suits: Vec<Suit> = vec![Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    let mut cards = Vec::new();
    for suit in suits {
        for rank in &ranks {
            cards.push(Card { suit, rank: *rank });
        }
    }
    shuffle(&mut cards);
    cards
}

/// Shuffles a `Vector` of cards in place, see [`rand::thread_rng::shuffle`](https://docs.rs/rand/0.5.4/rand/trait.Rng.html#method.shuffle).
pub fn shuffle(cards: &mut [Card]) {
    let mut rng = thread_rng();
    rng.shuffle(cards);
}

/// Used to reshuffle a deck of cards, panics if the `cards` does not have 52 elements (should only be used on a "full" deck).
pub fn deal_four_players(cards: &mut Vec<Card>) -> Vec<Vec<Card>> {
    assert_eq!(cards.len(), 52);
    shuffle(cards);
    let mut hands = [vec![], vec![], vec![], vec![]];

    let mut i = 0;
    while let Some(card) = cards.pop() {
        hands[i].push(card);
        i = (i + 1) % 4;
    }
    hands.to_vec()
}

#[cfg(test)]
mod tests {

    use cards::{
        deal_four_players, get_trick_winner, new_deck, shuffle, Card, Rank, Suit,
    };
    use std::fmt;

    #[test]
    fn shuffle_changes_cards() {
        let ah = Card::new(Suit::Heart, Rank::Ace);
        let ks = Card::new(Suit::Spade, Rank::King);
        let qc = Card::new(Suit::Club, Rank::Queen);
        let jd = Card::new(Suit::Diamond, Rank::Jack);
        let c2d = Card::new(Suit::Diamond, Rank::Two);
        let c3d = Card::new(Suit::Diamond, Rank::Three);
        let blank = Card::new(Suit::Blank, Rank::Blank);
        let mut cards = [ah, ks, qc, jd, c2d];
        let the_copy = cards;
        let the_clone = cards.clone();
        assert_eq!(cards, the_copy);
        assert_eq!(cards, the_clone);
        shuffle(&mut cards);
        assert!(cards.contains(&ah));
        assert!(cards.contains(&ks));
        assert!(cards.contains(&qc));
        assert!(cards.contains(&jd));
        assert!(cards.contains(&c2d));
        assert!(!cards.contains(&c3d));
        assert!(!cards.contains(&blank));
        assert_ne!(cards, the_copy);
        assert_ne!(cards, the_clone);
    }

    #[test]
    fn card_to_string() {
        let ah = Card::new(Suit::Heart, Rank::Ace);
        let ks = Card::new(Suit::Spade, Rank::King);
        let qc = Card::new(Suit::Club, Rank::Queen);
        let jd = Card::new(Suit::Diamond, Rank::Jack);
        let c2d = Card::new(Suit::Diamond, Rank::Two);
        assert_eq!(ah.to_string(), "Ace Heart".to_string());
        assert_eq!(ks.to_string(), "King Spade".to_string());
        assert_eq!(qc.to_string(), "Queen Club".to_string());
        assert_eq!(jd.to_string(), "Jack Diamond".to_string());
        assert_eq!(c2d.to_string(), "Two Diamond".to_string());
    }

    #[test]
    fn deal_4() {
        let ah = Card {
            suit: Suit::Heart,
            rank: Rank::Ace,
        };
        let ranks = [
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ];
        let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];
        let mut deck = Vec::new();
        for r in ranks {
            for s in suits {
                deck.push(Card::new(s.into(), r.into()));
            }
        }
        let hands = deal_four_players(&mut deck);
        assert_eq!(13, hands[0].len());
        assert_eq!(13, hands[1].len());
        assert_eq!(13, hands[2].len());
        assert_eq!(13, hands[3].len());
        assert_ne!(hands[0], hands[1]);
        assert_ne!(hands[0][0], hands[1][0]);
        assert!(
            hands[0].contains(&ah)
                || hands[1].contains(&ah)
                || hands[2].contains(&ah)
                || hands[3].contains(&ah)
        );
    }

    #[test]
    fn new_deck_test() {
        let ah = Card::new(Suit::Heart, Rank::Ace);
        let ks = Card::new(Suit::Spade, Rank::King);
        let qc = Card::new(Suit::Club, Rank::Queen);
        let jd = Card::new(Suit::Diamond, Rank::Jack);
        let c2d = Card::new(Suit::Diamond, Rank::Two);
        let c3d = Card::new(Suit::Diamond, Rank::Three);
        let blank = Card::new(Suit::Blank, Rank::Blank);
        let blank_spade = Card::new(Suit::Spade, Rank::Blank);
        let blank_3 = Card::new(Suit::Blank, Rank::Three);

        let deck = new_deck();
        assert!(deck.contains(&ah));
        assert!(deck.contains(&ks));
        assert!(deck.contains(&qc));
        assert!(deck.contains(&jd));
        assert!(deck.contains(&c2d));
        assert!(deck.contains(&c3d));
        assert!(!deck.contains(&blank));
        assert!(!deck.contains(&blank_spade));
        assert!(!deck.contains(&blank_3));
    }

    #[test]
    fn test_winner_of_tricks() {
        let ah = Card::new(Suit::Heart, Rank::Ace);
        let ks = Card::new(Suit::Spade, Rank::King);
        let qc = Card::new(Suit::Club, Rank::Queen);
        let jd = Card::new(Suit::Diamond, Rank::Jack);
        let c2d = Card::new(Suit::Diamond, Rank::Two);
        let c3d = Card::new(Suit::Diamond, Rank::Three);
        let c3s = Card::new(Suit::Spade, Rank::Three);

        let hand1 = [Some(c2d), Some(c3d), Some(jd), Some(qc)];
        assert_eq!(Some(2), get_trick_winner(0, &hand1));
        assert_eq!(Some(2), get_trick_winner(1, &hand1));
        assert_eq!(Some(2), get_trick_winner(2, &hand1));
        assert_eq!(Some(3), get_trick_winner(3, &hand1));

        let hand2 = [Some(ah), Some(ks), Some(qc), Some(jd)];
        assert_eq!(Some(1), get_trick_winner(0, &hand2));
        assert_eq!(Some(1), get_trick_winner(1, &hand2));
        assert_eq!(Some(1), get_trick_winner(2, &hand2));
        assert_eq!(Some(1), get_trick_winner(3, &hand2));

        let hand3 = [Some(ah), Some(c3d), Some(qc), Some(jd)];
        assert_eq!(Some(0), get_trick_winner(0, &hand3));
        assert_eq!(Some(3), get_trick_winner(1, &hand3));
        assert_eq!(Some(2), get_trick_winner(2, &hand3));
        assert_eq!(Some(3), get_trick_winner(3, &hand3));

        let hand4 = [Some(ah), Some(c3s), Some(qc), Some(jd)];
        assert_eq!(Some(1), get_trick_winner(0, &hand4));
        assert_eq!(Some(1), get_trick_winner(1, &hand4));
        assert_eq!(Some(1), get_trick_winner(2, &hand4));
        assert_eq!(Some(1), get_trick_winner(3, &hand4));

        let hand5 = [Some(ks), Some(c3s), Some(qc), Some(jd)];
        assert_eq!(Some(0), get_trick_winner(0, &hand5));
        assert_eq!(Some(0), get_trick_winner(1, &hand5));
        assert_eq!(Some(0), get_trick_winner(2, &hand5));
        assert_eq!(Some(0), get_trick_winner(3, &hand5));
    }
}
