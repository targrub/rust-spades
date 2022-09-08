#![allow(unused)]

extern crate rand;

use self::rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::fmt::{self, Display};

#[derive(Default, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Debug, Hash)]
pub enum Suit {
    #[default]
    Clubs = 0,
    Diamonds = 1,
    Hearts = 2,
    Spades = 3,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suit::Clubs => write!(f, "\u{2663}"),
            Suit::Diamonds => write!(f, "\u{2666}"),
            Suit::Hearts => write!(f, "\u{2665}"),
            Suit::Spades => write!(f, "\u{2660}"),
        }
    }
}

impl From<u8> for Suit {
    fn from(f: u8) -> Self {
        match f {
            0 => Suit::Clubs,
            1 => Suit::Diamonds,
            2 => Suit::Hearts,
            3 => Suit::Spades,
            _ => panic!("illegal suit"),
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Debug, Hash)]
pub enum Rank {
    #[default]
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
            _ => panic!("illegal rank"),
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
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
        write!(f, "{}{}", self.rank, self.suit)
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

impl serde::Serialize for Card {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(self.rank as u8 + 15 * (self.suit as u8))
    }
}

struct U8Visitor;

impl<'de> serde::de::Visitor<'de> for U8Visitor {
    type Value = Card;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between 2 and 62")
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Card {
            rank: (value % 15).into(),
            suit: (value / 15).into(),
        })
    }
}

impl<'de> serde::Deserialize<'de> for Card {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u8(U8Visitor)
    }
}

#[test]
fn test_ser_de() {
    let mut card = Card::new(Suit::Diamonds, Rank::King);
    serde_test::assert_tokens(&card, &[serde_test::Token::U8(15 + 13)]);
    card = Card::new(Suit::Clubs, Rank::Two);
    serde_test::assert_tokens(&card, &[serde_test::Token::U8(0 + 2)]);
    card = Card::new(Suit::Spades, Rank::Ace);
    serde_test::assert_tokens(&card, &[serde_test::Token::U8(15 * 3 + 14)]);
}

/// Given four cards and a starting card, returns the winner of a trick.
///
/// The rules used to determine the winner of a trick are as follows:
/// * Spades trump all other suits
/// * The suit the first player (given by index) plays sets the suit of the trick
/// * The highest ranking spades card or card of suit of first player's card wins the trick.
/// Note: assumes leading card is valid (e.g., if non-spade led and not broken spades, this method doesn't care)
pub fn get_trick_winner(leading_player_index: usize, others: &Vec<Card>) -> usize {
    assert_eq!(4, others.len());
    let mut winning_index = 0;
    let mut best_card = others[0];
    for (i, other) in others.iter().enumerate() {
        if other.suit == best_card.suit {
            if other.rank as u8 > best_card.rank as u8 {
                best_card = *other;
                winning_index = i;
            }
        } else if other.suit == Suit::Spades {
            best_card = *other;
            winning_index = i;
        }
    }
    (winning_index + leading_player_index) % 4
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
    let suits: Vec<Suit> = vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

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
mod suit_tests {
    use super::Suit;

    #[test]
    fn test_from_u8_to_suit() {
        let mut s: Suit = 2u8.into();
        assert_eq!(Suit::Hearts, s);
        assert_eq!(Suit::Clubs, 0u8.into());
        assert_eq!(Suit::Diamonds, 1u8.into());
        assert_eq!(Suit::Hearts, 2u8.into());
        assert_eq!(Suit::Spades, 3u8.into());
    }

    #[test]
    #[should_panic(expected = "illegal suit")]
    fn test_from_4_to_suit_panics() {
        let s: Suit = 4u8.into();
    }

    #[test]
    #[should_panic(expected = "illegal suit")]
    fn test_from_5_to_suit_panics() {
        let s: Suit = 5u8.into();
    }
}

#[cfg(test)]
mod rank_tests {

    use cards::Rank;

    #[test]
    fn test_from_u8_to_rank() {
        let r: Rank = 3u8.into();
        assert_eq!(Rank::Three, r);
        assert_eq!(Rank::Ace, 14u8.into());
        assert_eq!(Rank::Ten, 10u8.into());
        assert_eq!(Rank::Two, 2u8.into());
    }

    #[test]
    #[should_panic(expected = "illegal rank")]
    fn test_from_0_to_rank() {
        let r: Rank = 0u8.into();
    }

    #[test]
    #[should_panic(expected = "illegal rank")]
    fn test_from_1_to_rank() {
        let r: Rank = 1u8.into();
    }

    #[test]
    #[should_panic(expected = "illegal rank")]
    fn test_from_15_to_rank() {
        let r: Rank = 15u8.into();
    }
}

#[cfg(test)]
mod tests {

    use cards::{deal_four_players, get_trick_winner, new_deck, shuffle, Card, Rank, Suit};
    use std::fmt;

    #[test]
    fn shuffle_changes_cards() {
        let ah = Card::new(Suit::Hearts, Rank::Ace);
        let ks = Card::new(Suit::Spades, Rank::King);
        let qc = Card::new(Suit::Clubs, Rank::Queen);
        let jd = Card::new(Suit::Diamonds, Rank::Jack);
        let c2d = Card::new(Suit::Diamonds, Rank::Two);
        let c3d = Card::new(Suit::Diamonds, Rank::Three);
        let c4d = Card::new(Suit::Diamonds, Rank::Four);
        let c5d = Card::new(Suit::Diamonds, Rank::Five);
        let c6d = Card::new(Suit::Diamonds, Rank::Six); // NOTE! not in cards vec
        let mut cards = [ah, ks, qc, jd, c2d, c3d, c4d, c5d];
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
        assert!(cards.contains(&c3d));
        assert!(cards.contains(&c4d));
        assert!(cards.contains(&c5d));
        assert!(!cards.contains(&c6d));
        assert_ne!(cards, the_copy);
        assert_ne!(cards, the_clone);
    }

    #[test]
    fn card_to_string() {
        let ah = Card::new(Suit::Hearts, Rank::Ace);
        let ks = Card::new(Suit::Spades, Rank::King);
        let qc = Card::new(Suit::Clubs, Rank::Queen);
        let jd = Card::new(Suit::Diamonds, Rank::Jack);
        let c2d = Card::new(Suit::Diamonds, Rank::Two);
        assert_eq!(ah.to_string(), "A\u{2665}".to_string());
        assert_eq!(ks.to_string(), "K\u{2660}".to_string());
        assert_eq!(qc.to_string(), "Q\u{2663}".to_string());
        assert_eq!(jd.to_string(), "J\u{2666}".to_string());
        assert_eq!(c2d.to_string(), "2\u{2666}".to_string());
    }

    #[test]
    fn deal_4() {
        let ah = Card {
            suit: Suit::Hearts,
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
        let suits = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
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
        let ah = Card::new(Suit::Hearts, Rank::Ace);
        let ks = Card::new(Suit::Spades, Rank::King);
        let qc = Card::new(Suit::Clubs, Rank::Queen);
        let jd = Card::new(Suit::Diamonds, Rank::Jack);
        let c2d = Card::new(Suit::Diamonds, Rank::Two);
        let c3d = Card::new(Suit::Diamonds, Rank::Three);

        let deck = new_deck();
        assert!(deck.contains(&ah));
        assert!(deck.contains(&ks));
        assert!(deck.contains(&qc));
        assert!(deck.contains(&jd));
        assert!(deck.contains(&c2d));
        assert!(deck.contains(&c3d));
    }

    #[test]
    fn test_winner_of_tricks() {
        let ah = Card::new(Suit::Hearts, Rank::Ace);
        let ks = Card::new(Suit::Spades, Rank::King);
        let qc = Card::new(Suit::Clubs, Rank::Queen);
        let jd = Card::new(Suit::Diamonds, Rank::Jack);
        let c2d = Card::new(Suit::Diamonds, Rank::Two);
        let c3d = Card::new(Suit::Diamonds, Rank::Three);
        let c3s = Card::new(Suit::Spades, Rank::Three);

        let hand1 = vec![c2d, c3d, jd, qc];
        assert_eq!(2, get_trick_winner(0, &hand1));
        assert_eq!(3, get_trick_winner(1, &hand1));
        assert_eq!(0, get_trick_winner(2, &hand1));
        assert_eq!(1, get_trick_winner(3, &hand1));

        let hand2 = vec![ah, ks, qc, jd];
        assert_eq!(1, get_trick_winner(0, &hand2));
        assert_eq!(2, get_trick_winner(1, &hand2));
        assert_eq!(3, get_trick_winner(2, &hand2));
        assert_eq!(0, get_trick_winner(3, &hand2));

        let hand3 = vec![c3d, qc, jd, ah];
        assert_eq!(2, get_trick_winner(0, &hand3));
        assert_eq!(3, get_trick_winner(1, &hand3));
        assert_eq!(0, get_trick_winner(2, &hand3));
        assert_eq!(1, get_trick_winner(3, &hand3));

        let hand4 = vec![ah, c3s, qc, jd];
        assert_eq!(1, get_trick_winner(0, &hand4));
        assert_eq!(2, get_trick_winner(1, &hand4));
        assert_eq!(3, get_trick_winner(2, &hand4));
        assert_eq!(0, get_trick_winner(3, &hand4));

        let hand5 = vec![ks, c3s, qc, jd];
        assert_eq!(0, get_trick_winner(0, &hand5));
        assert_eq!(1, get_trick_winner(1, &hand5));
        assert_eq!(2, get_trick_winner(2, &hand5));
        assert_eq!(3, get_trick_winner(3, &hand5));
    }
}
