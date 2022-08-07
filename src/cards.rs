#![allow(unused)]

extern crate rand;

use self::rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::fmt::{self, Display};

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Debug, Hash)]
pub enum Suit {
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Debug, Hash)]
pub enum Rank {
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
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
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
pub fn get_trick_winner(leading_player_index: usize, others: &[Card; 4]) -> usize {
    let mut winning_index = leading_player_index;
    let mut best_card = &others[leading_player_index];

    for (i, other) in others.iter().enumerate() {
        if other.suit == best_card.suit {
            if other.rank as u8 > best_card.rank as u8 {
                best_card = other;
                winning_index = i;
            }
        } else if other.suit == Suit::Spade {
            best_card = other;
            winning_index = i;
        }
    }
    winning_index
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

/// Returns an array of `Blank` suited and ranked cards.
pub fn new_pot() -> [Card; 4] {
    [
        Card {
            suit: Suit::Blank,
            rank: Rank::Blank,
        },
        Card {
            suit: Suit::Blank,
            rank: Rank::Blank,
        },
        Card {
            suit: Suit::Blank,
            rank: Rank::Blank,
        },
        Card {
            suit: Suit::Blank,
            rank: Rank::Blank,
        },
    ]
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
        deal_four_players, get_trick_winner, new_deck, new_pot, shuffle, Card, Rank, Suit,
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
    fn new_pot_test() {
        let c3d = Card::new(Suit::Diamond, Rank::Three);
        let blank = Card::new(Suit::Blank, Rank::Blank);
        let blank_spade = Card::new(Suit::Spade, Rank::Blank);
        let blank_3 = Card::new(Suit::Blank, Rank::Three);

        let cards = new_pot();
        assert!(!cards.contains(&c3d));
        assert!(cards.contains(&blank));
        assert!(!cards.contains(&blank_spade));
        assert!(!cards.contains(&blank_3));
        assert_eq!(blank, cards[0]);
        assert_eq!(blank, cards[1]);
        assert_eq!(blank, cards[2]);
        assert_eq!(blank, cards[3]);
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

        let hand1 = [c2d, c3d, jd, qc];
        assert_eq!(2, get_trick_winner(0, &hand1));
        assert_eq!(2, get_trick_winner(1, &hand1));
        assert_eq!(2, get_trick_winner(2, &hand1));
        assert_eq!(3, get_trick_winner(3, &hand1));

        let hand2 = [ah, ks, qc, jd];
        assert_eq!(1, get_trick_winner(0, &hand2));
        assert_eq!(1, get_trick_winner(1, &hand2));
        assert_eq!(1, get_trick_winner(2, &hand2));
        assert_eq!(1, get_trick_winner(3, &hand2));

        let hand3 = [ah, c3d, qc, jd];
        assert_eq!(0, get_trick_winner(0, &hand3));
        assert_eq!(3, get_trick_winner(1, &hand3));
        assert_eq!(2, get_trick_winner(2, &hand3));
        assert_eq!(3, get_trick_winner(3, &hand3));

        let hand4 = [ah, c3s, qc, jd];
        assert_eq!(1, get_trick_winner(0, &hand4));
        assert_eq!(1, get_trick_winner(1, &hand4));
        assert_eq!(1, get_trick_winner(2, &hand4));
        assert_eq!(1, get_trick_winner(3, &hand4));

        let hand5 = [ks, c3s, qc, jd];
        assert_eq!(0, get_trick_winner(0, &hand5));
        assert_eq!(0, get_trick_winner(1, &hand5));
        assert_eq!(0, get_trick_winner(2, &hand5));
        assert_eq!(0, get_trick_winner(3, &hand5));
    }
}
