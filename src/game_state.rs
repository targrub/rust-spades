use std::fmt::Display;

/// Current game stage, field of `Game`.
///
/// The `Betting` and `Trick` variants have a `usize` value between 0
/// and 3, inclusive, that refers to the number of players that have placed bets or played cards in the trick,
/// respectively.
///
/// **Example:** `State::Trick(2)` means the game is in the card playing stage, and two players have played their cards.
#[derive(Debug, Default, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
pub enum State {
    #[default]
    GameNotStarted,
    Betting(usize),
    Trick(usize),
    GameCompleted,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
