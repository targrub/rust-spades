use std::fmt;

#[derive(Debug, PartialEq)]
pub enum SpadesError {
    InvalidUuid,
    GameNotStarted,
    GameCompleted,
    GameNotCompleted,
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

#[derive(PartialEq)]
pub enum TransitionSuccess {
    Bet,
    BetComplete,
    Trick,
    PlayCard,
    GameOver,
    Start,
}

#[derive(PartialEq)]
pub enum TransitionError {
    GameAlreadyStarted,
    GameNotStarted,
    BetInCompletedGame,
    BetNotInBettingStage,
    CardInBettingStage,
    CardInCompletedGame,
    CardNotInHand,
    CardIncorrectSuit,
}

impl fmt::Display for TransitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            TransitionError::GameAlreadyStarted => {
                write!(f, "attempted to start a game that is already started")
            }
            TransitionError::GameNotStarted => {
                write!(f, "attempted to play a game that has not yet started")
            }
            TransitionError::BetInCompletedGame => {
                write!(f, "attempted to make bet while game is completed")
            }
            TransitionError::BetNotInBettingStage => {
                write!(
                    f,
                    "attempted to place a bet while game is not in betting stage"
                )
            }
            TransitionError::CardInBettingStage => {
                write!(f, "attempted to play a card while game is in betting stage")
            }
            TransitionError::CardInCompletedGame => {
                write!(f, "attempted to play a card while game is completed")
            }
            TransitionError::CardNotInHand => {
                write!(f, "attempted to play a card not in hand")
            }
            TransitionError::CardIncorrectSuit => {
                write!(f, "attempted to play a card of the wrong suit")
            }
        }
    }
}
