use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TransitionSuccess {
    Bet,
    BetComplete,
    Trick,
    PlayCard,
    GameOver,
    Start,
}

#[derive(Debug, PartialEq)]
pub enum SpadesError {
    InvalidUuid,
    GameNotStarted,
    GameCompleted,
    GameNotCompleted,
    Unknown,
}

impl fmt::Display for SpadesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            SpadesError::InvalidUuid => {
                write!(f, "Error: Attempted to retrieve by an invalid Uuid")
            }
            SpadesError::GameNotStarted => {
                write!(f, "Error: Game not started yet.")
            }
            SpadesError::GameCompleted => {
                write!(f, "Error: Game is completed.")
            }
            SpadesError::GameNotCompleted => {
                write!(f, "Error: Game is still ongoing.")
            }
            SpadesError::Unknown => {
                write!(f, "Error: Unknown get error occurred.")
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TransitionError {
    GameAlreadyStarted,
    GameNotStarted,
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
