#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Action(usize, usize);

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum Fitness {
    Loss,
    /// the game is still ongoing or a tie
    Even,
    Win,
}

impl rubot::Game for Game {
    type Player = Piece;
    type Action = Action;
    type Actions = Vec<Action>;
    type Fitness = Fitness;

    fn actions(&self, player: Self::Player) -> (bool, Self::Actions) {
        let mut actions = Vec::new();
        if !self.is_finished() {
            for x in 0..3 {
                for y in 0..3 {
                    if let None = self.tiles()[x][y] {
                        actions.push(Action(x, y));
                    }
                }
            }
        }
        (player == self.current_piece(), actions)
    }

    fn execute(&mut self, action: &Self::Action, player: Self::Player) -> Self::Fitness {
        match self.make_move(action.0, action.1) {
            Ok(()) => (),
            Err(e) => unreachable!("Error: {:?}", e),
        }

        match self.winner() {
            None | Some(Winner::Tie) => Fitness::Even,
            Some(Winner::O) => {
                if player == Piece::O {
                    Fitness::Win
                } else {
                    Fitness::Loss
                }
            }
            Some(Winner::X) => {
                if player == Piece::X {
                    Fitness::Win
                } else {
                    Fitness::Loss
                }
            }
        }
    }

    fn is_upper_bound(&self, fitness: Self::Fitness, _: Self::Player) -> bool {
        fitness == Fitness::Win
    }

    fn is_lower_bound(&self, fitness: Self::Fitness, _: Self::Player) -> bool {
        fitness == Fitness::Loss
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rubot::{Bot, Logger, ToCompletion};

    #[test]
    fn first_pos() {
        let mut game = Game::new();
        game.make_move(0, 0).unwrap();

        let mut opponent = Bot::new(Piece::O);
        let mut logger = Logger::new(ToCompletion);
        assert_eq!(opponent.select(&game, &mut logger).unwrap(), Action(1, 1));
        assert!(logger.duration() < Duration::from_secs(1));
    }
}
