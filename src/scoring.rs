use std::fmt;
use cards::{get_trick_winner, Card};

/// Used as an argument to [Game::place_bet](struct.Game.html#method.place_bet).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Bet {
    Amount(u8),
    Nil,
    BlindNil,
}

impl fmt::Display for Bet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct GameConfig {
    max_points: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PlayerState {
    won_trick: [bool; 13],
}

impl PlayerState {
    fn new() -> PlayerState {
        PlayerState {
            won_trick: [false; 13],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TeamState {
    tricks: u8,
    game_bags: u8,
    cumulative_bags: u8,
    game_points: i32,
    cumulative_points: i32,
}

impl fmt::Display for TeamState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TeamState {
    fn new() -> TeamState {
        TeamState {
            tricks: 0,
            game_bags: 0,
            cumulative_bags: 0,
            game_points: 0,
            cumulative_points: 0,
        }
    }

    pub fn tricks_won(&self) -> u8 {
        self.tricks
    }

    pub fn game_bags(&self) -> u8 {
        self.game_bags
    }

    pub fn cumulative_bags(&self) -> u8 {
        self.cumulative_bags
    }

    pub fn game_points(&self) -> i32 {
        self.game_points
    }

    pub fn cumulative_points(&self) -> i32 {
        self.cumulative_points
    }

    fn calculate_round_totals(
        &mut self,
        first_bet: Bet,
        first_player: &PlayerState,
        second_bet: Bet,
        second_player: &PlayerState,
    ) {
        let first_player_tricks = first_player.won_trick.iter().filter(|x| **x).count() as u8;
        let second_player_tricks = second_player.won_trick.iter().filter(|x| **x).count() as u8;
        self.tricks = (first_player_tricks + second_player_tricks) as u8;
        let first_player_bet = {
            match first_bet {
                Bet::Amount(amount) => amount,
                Bet::Nil => 0,
                Bet::BlindNil => 0,
            }
        };
        let second_player_bet = {
            match second_bet {
                Bet::Amount(amount) => amount,
                Bet::Nil => 0,
                Bet::BlindNil => 0,
            }
        };
        let team_bets = first_player_bet + second_player_bet;
        assert!(first_player_tricks <= 13);
        assert!(second_player_tricks <= 13);
        if self.tricks > 13 {
            panic!("won over 13 tricks: got {}", self.tricks);
        }
        self.game_points = 0;
        self.game_bags = 0;
        if self.tricks >= team_bets {
            let game_bags = self.tricks - team_bets;
            assert!(game_bags <= 13);
            self.game_bags = game_bags;
            if first_player_bet != 0 && second_player_bet != 0 {
                self.game_points += self.tricks as i32 - team_bets as i32 + (team_bets as i32 * 10);
            }
        } else {
            self.game_points -= (team_bets as i32) * 10;
        }

        if first_player_bet == 0 {
            if first_player_tricks == 0 {
                self.game_points += 100;
            } else {
                self.game_points -= 100;
            }
            if second_player_tricks >= team_bets && second_player_bet != 0 {
                self.game_points += self.tricks as i32 - team_bets as i32 + (team_bets as i32 * 10);
            }
        }
        if second_player_bet == 0 {
            if second_player_tricks == 0 {
                self.game_points += 100;
            } else {
                self.game_points -= 100;
            }
            if first_player_tricks >= team_bets && first_player_bet != 0 {
                self.game_points += self.tricks as i32 - team_bets as i32 + (team_bets as i32 * 10);
            }
        }
        self.cumulative_bags += self.game_bags;

        if self.cumulative_bags >= 10 {
            self.cumulative_bags -= 10;
            self.game_points -= 100;
        }
        self.cumulative_points += self.game_points;
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Scoring {
    config: GameConfig,
    pub team: [TeamState; 2],
    players: [PlayerState; 4],
    in_betting_stage: bool,
    bets_placed: [Bet; 4],
    is_over: bool,
    round: usize,
    trick: usize,
}

impl Scoring {
    pub fn new(max_points: i32) -> Scoring {
        Scoring {
            team: [TeamState::new(), TeamState::new()],
            in_betting_stage: true,
            players: [PlayerState::new(); 4],
            bets_placed: [Bet::Amount(0); 4],
            is_over: false,
            round: 0,
            trick: 0,
            config: GameConfig { max_points },
        }
    }

    pub fn add_bet(&mut self, current_player_index: usize, bet: Bet) {
        self.bets_placed[current_player_index] = bet;
    }

    pub fn betting_over(&mut self) {
        self.trick = 0;
        self.in_betting_stage = false;
        for mut p in &mut self.players {
            for i in 0..13 {
                p.won_trick[i] = false;
            }
        }
        self.team[0].game_bags = 0;
        self.team[1].game_bags = 0;
        self.team[0].game_points = 0;
        self.team[1].game_points = 0;
    }

    pub fn trick(&mut self, starting_player_index: usize, cards: &[Card; 4]) -> usize {
        let winner = get_trick_winner(starting_player_index, cards);
        self.players[winner].won_trick[self.trick] = true;

        if self.trick == 12 {
            // score the round
            self.team[0].calculate_round_totals(
                self.bets_placed[0],
                &self.players[0],
                self.bets_placed[2],
                &self.players[2],
            );
            self.team[1].calculate_round_totals(
                self.bets_placed[1],
                &self.players[1],
                self.bets_placed[3],
                &self.players[3],
            );
            if self.team[0].cumulative_points >= self.config.max_points
                || self.team[1].cumulative_points >= self.config.max_points
            {
                self.is_over = true;
            }

            // reset structure for possible next round
            self.in_betting_stage = true;

            self.round += 1;
        } else {
            self.trick += 1;
        }
        winner
    }

    pub fn is_over(&self) -> bool {
        self.is_over
    }

    pub fn is_in_betting_stage(&self) -> bool {
        self.in_betting_stage
    }
}

#[cfg(test)]
mod tests {
    use super::{PlayerState, Scoring, TeamState};
    use Bet;

    #[test]
    fn test_playerstate_new() {
        let ps = PlayerState::new();
        for i in 0..13 {
            assert_eq!(false, ps.won_trick[i]);
        }
        assert_eq!(13, ps.won_trick.len());
    }

    #[test]
    fn test_scoring_new_with_negative_max() {
        let sc = Scoring::new(-32);
        assert_eq!(-32, sc.config.max_points)
    }

    #[test]
    fn test_scoring_new_with_max_of_0() {
        let sc = Scoring::new(0);
        assert_eq!(0, sc.config.max_points)
    }

    #[test]
    fn test_scoring_new_with_positive_max() {
        let sc = Scoring::new(320);
        assert_eq!(320, sc.config.max_points)
    }

    #[test]
    fn test_game_end_scoring_zero_overtricks() {
        let mut ts = TeamState::new();
        let first_bet = Bet::Amount(3);
        let second_bet = Bet::Amount(8);
        let mut first_player = PlayerState::new();
        let second_player = PlayerState::new();
        for i in 0..11 {
            first_player.won_trick[i] = true;
        }
        ts.calculate_round_totals(first_bet, &first_player, second_bet, &second_player);
        assert_eq!(0, ts.game_bags());
        assert_eq!(0, ts.cumulative_bags());
        assert_eq!(110, ts.game_points());
        assert_eq!(110, ts.cumulative_points());
        assert_eq!(11, ts.tricks_won());
    }

    #[test]
    fn test_game_end_scoring_zero_overtricks_betnil_succeeds() {
        let mut ts = TeamState::new();
        let first_bet = Bet::Amount(11);
        let second_bet = Bet::Nil;
        let mut first_player = PlayerState::new();
        let second_player = PlayerState::new();
        for i in 0..11 {
            first_player.won_trick[i] = true;
        }
        ts.calculate_round_totals(first_bet, &first_player, second_bet, &second_player);
        assert_eq!(0, ts.game_bags());
        assert_eq!(0, ts.cumulative_bags());
        assert_eq!(210, ts.game_points());
        assert_eq!(210, ts.cumulative_points());
        assert_eq!(11, ts.tricks_won());
    }

    #[test]
    fn test_game_end_scoring_zero_overtricks_betnil_fails() {
        let mut ts = TeamState::new();
        let first_bet = Bet::Nil;
        let second_bet = Bet::Amount(11);
        let mut first_player = PlayerState::new();
        let second_player = PlayerState::new();
        for i in 0..11 {
            first_player.won_trick[i] = true;
        }
        ts.calculate_round_totals(first_bet, &first_player, second_bet, &second_player);
        assert_eq!(0, ts.game_bags());
        assert_eq!(0, ts.cumulative_bags());
        assert_eq!(-100, ts.game_points());
        assert_eq!(-100, ts.cumulative_points());
        assert_eq!(11, ts.tricks_won());
    }

    #[test]
    fn test_game_end_scoring_one_overtrick() {
        let mut ts = TeamState::new();
        let first_bet = Bet::Amount(3);
        let second_bet = Bet::Amount(8);
        let mut first_player = PlayerState::new();
        let second_player = PlayerState::new();
        for i in 0..12 {
            first_player.won_trick[i] = true;
        }
        ts.calculate_round_totals(first_bet, &first_player, second_bet, &second_player);
        assert_eq!(1, ts.game_bags());
        assert_eq!(1, ts.cumulative_bags());
        assert_eq!(111, ts.game_points());
        assert_eq!(111, ts.cumulative_points());
        assert_eq!(12, ts.tricks_won());
    }

    #[test]
    fn test_game_end_scoring_two_overtricks() {
        let mut ts = TeamState::new();
        let first_bet = Bet::Amount(3);
        let second_bet = Bet::Amount(8);
        let mut first_player = PlayerState::new();
        let second_player = PlayerState::new();
        for i in 0..13 {
            first_player.won_trick[i] = true;
        }
        ts.calculate_round_totals(first_bet, &first_player, second_bet, &second_player);
        assert_eq!(2, ts.game_bags());
        assert_eq!(2, ts.cumulative_bags());
        assert_eq!(112, ts.game_points());
        assert_eq!(112, ts.cumulative_points());
        assert_eq!(13, ts.tricks_won());
    }

    #[test]
    fn test_game_end_scoring_two_overtricks_betnil_succeeds() {
        let mut ts = TeamState::new();
        let first_bet = Bet::Amount(11);
        let second_bet = Bet::Nil;
        let mut first_player = PlayerState::new();
        let second_player = PlayerState::new();
        for i in 0..13 {
            first_player.won_trick[i] = true;
        }
        ts.calculate_round_totals(first_bet, &first_player, second_bet, &second_player);
        assert_eq!(2, ts.game_bags());
        assert_eq!(2, ts.cumulative_bags());
        assert_eq!(112 + 100, ts.game_points());
        assert_eq!(112 + 100, ts.cumulative_points());
        assert_eq!(13, ts.tricks_won());
    }

    #[test]
    fn test_game_end_scoring_two_overtricks_betnil_fails() {
        let mut ts = TeamState::new();
        let first_bet = Bet::Nil;
        let second_bet = Bet::Amount(11);
        let mut first_player = PlayerState::new();
        let second_player = PlayerState::new();
        for i in 0..13 {
            first_player.won_trick[i] = true;
        }
        ts.calculate_round_totals(first_bet, &first_player, second_bet, &second_player);
        assert_eq!(2, ts.game_bags());
        assert_eq!(2, ts.cumulative_bags());
        assert_eq!(-100, ts.game_points());
        assert_eq!(-100, ts.cumulative_points());
        assert_eq!(13, ts.tricks_won());
    }

    #[test]
    fn test_game_end_scoring_bet_all_win_all() {
        let mut ts = TeamState::new();
        let first_bet = Bet::Amount(13);
        let second_bet = Bet::Nil;
        let mut first_player = PlayerState::new();
        let second_player = PlayerState::new();
        for i in 0..13 {
            first_player.won_trick[i] = true;
        }
        ts.calculate_round_totals(first_bet, &first_player, second_bet, &second_player);
        assert_eq!(0, ts.game_bags());
        assert_eq!(0, ts.cumulative_bags());
        assert_eq!(230, ts.game_points());
        assert_eq!(230, ts.cumulative_points());
        assert_eq!(13, ts.tricks_won());
    }

    #[test]
    fn test_game_end_scoring_bet_all_fall_short() {
        let mut ts = TeamState::new();
        let first_bet = Bet::Amount(13);
        let second_bet = Bet::Nil;
        let mut first_player = PlayerState::new();
        let second_player = PlayerState::new();
        for i in 0..12 {
            first_player.won_trick[i] = true;
        }
        ts.calculate_round_totals(first_bet, &first_player, second_bet, &second_player);
        assert_eq!(0, ts.game_bags());
        assert_eq!(0, ts.cumulative_bags());
        assert_eq!(-130 + 100, ts.game_points());
        assert_eq!(-130 + 100, ts.cumulative_points());
        assert_eq!(12, ts.tricks_won());
    }

    #[test]
    fn test_game_end_scoring_bet_all_wrongly_fall_short() {
        let mut ts = TeamState::new();
        let first_bet = Bet::Nil;
        let second_bet = Bet::Amount(13);
        let mut first_player = PlayerState::new();
        let second_player = PlayerState::new();
        for i in 0..12 {
            first_player.won_trick[i] = true;
        }
        ts.calculate_round_totals(first_bet, &first_player, second_bet, &second_player);
        assert_eq!(0, ts.game_bags());
        assert_eq!(0, ts.cumulative_bags());
        assert_eq!(-130 - 100, ts.game_points());
        assert_eq!(-130 - 100, ts.cumulative_points());
        assert_eq!(12, ts.tricks_won());
    }

    #[test]
    fn test_game_end_scoring_bet_all_wrongly_fall_very_short() {
        let mut ts = TeamState::new();
        let first_bet = Bet::Nil;
        let second_bet = Bet::Amount(13);
        let first_player = PlayerState::new();
        let second_player = PlayerState::new();
        ts.calculate_round_totals(first_bet, &first_player, second_bet, &second_player);
        assert_eq!(0, ts.game_bags());
        assert_eq!(0, ts.cumulative_bags());
        assert_eq!(-130 + 100, ts.game_points());
        assert_eq!(-130 + 100, ts.cumulative_points());
        assert_eq!(0, ts.tricks_won());
    }

    #[test]
    fn test_game_end_scoring_bet_completely_wrongly_fall_very_short() {
        let mut ts = TeamState::new();
        let first_bet = Bet::Amount(1);
        let second_bet = Bet::Amount(12);
        let first_player = PlayerState::new();
        let second_player = PlayerState::new();
        ts.calculate_round_totals(first_bet, &first_player, second_bet, &second_player);
        assert_eq!(0, ts.game_bags());
        assert_eq!(0, ts.cumulative_bags());
        assert_eq!(-130, ts.game_points());
        assert_eq!(-130, ts.cumulative_points());
        assert_eq!(0, ts.tricks_won());
    }

    #[test]
    fn test_game_end_scoring_complete_betting_fail() {
        let mut ts = TeamState::new();
        let first_bet = Bet::Nil;
        let second_bet = Bet::Nil;
        let mut first_player = PlayerState::new();
        let mut second_player = PlayerState::new();
        for i in 0..12 {
            first_player.won_trick[i] = true;
        }
        for i in 12..13 {
            second_player.won_trick[i] = true;
        }
        ts.calculate_round_totals(first_bet, &first_player, second_bet, &second_player);
        assert_eq!(13, ts.game_bags());
        assert_eq!(3, ts.cumulative_bags());
        assert_eq!(-300, ts.game_points());
        assert_eq!(-300, ts.cumulative_points());
        assert_eq!(13, ts.tricks_won());
    }

    #[test]
    fn test_game_end_scoring_each_bidnil_win_1() {
        let mut ts = TeamState::new();
        let first_bet = Bet::Nil;
        let second_bet = Bet::Nil;
        let mut first_player = PlayerState::new();
        let mut second_player = PlayerState::new();
        for i in 0..1 {
            first_player.won_trick[i] = true;
        }
        for i in 12..13 {
            second_player.won_trick[i] = true;
        }
        ts.calculate_round_totals(first_bet, &first_player, second_bet, &second_player);
        assert_eq!(2, ts.game_bags());
        assert_eq!(2, ts.cumulative_bags());
        assert_eq!(-200, ts.game_points());
        assert_eq!(-200, ts.cumulative_points());
        assert_eq!(2, ts.tricks_won());
    }

    #[test]
    #[should_panic(expected = "won over 13 tricks")]
    fn test_game_end_scoring_winning_14_tricks_panics() {
        let mut ts = TeamState::new();
        let first_bet = Bet::Nil;
        let second_bet = Bet::Nil;
        let mut first_player = PlayerState::new();
        let mut second_player = PlayerState::new();
        for i in 0..13 {
            first_player.won_trick[i] = true;
        }
        for i in 12..13 {
            second_player.won_trick[i] = true;
        }
        ts.calculate_round_totals(first_bet, &first_player, second_bet, &second_player);
    }
}
