use std::fmt;

#[derive(Debug, PartialEq)]
pub enum SpadesError {
    InvalidUuid,
    GameNotStarted,
    GameCompleted,
    GameNotCompleted,
    BetImproperSeenHand,
    CardIncorrectSuit,
    CardNotInHand,
    ImproperGameStage,
    InternalError, // error within library
}

impl fmt::Display for SpadesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            SpadesError::InvalidUuid => {
                write!(f, "invalid Uuid")
            }
            SpadesError::GameNotStarted => {
                write!(f, "game not started")
            }
            SpadesError::GameCompleted => {
                write!(f, "game is complete")
            }
            SpadesError::GameNotCompleted => {
                write!(f, "game is not complete")
            }
            SpadesError::BetImproperSeenHand => {
                write!(f, "blind nil bet improper; seen hand")
            }
            SpadesError::CardIncorrectSuit => {
                write!(f, "card of incorrect suit")
            }
            SpadesError::CardNotInHand => {
                write!(f, "card not in hand")
            }
            SpadesError::ImproperGameStage => {
                write!(f, "improper stage of game to take that action")
            }
            SpadesError::InternalError => {
                write!(f, "spades crate internal error")
            }
        }
    }
}
