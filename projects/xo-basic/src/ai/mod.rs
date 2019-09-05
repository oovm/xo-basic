use crate::{Game, Player, Winner};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Action(usize);

impl From<u32> for Action {
    fn from(u: u32) -> Self {
        Action(u as usize)
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum Fitness {
    Win,
    Loss,
    /// the game is still ongoing or a tie
    Even,
}

impl rubot::Game for Game {
    type Player = Player;
    type Action = Action;
    type Fitness = Fitness;
    type Actions = Vec<Action>;

    fn actions(&self, player: Self::Player) -> (bool, Self::Actions) {
        let mut actions = Vec::new();
        if !self.is_finished() {
            actions = self.available_moves().iter().map(|e| Action(*e)).collect()
        }
        (player == self.player, actions)
    }

    fn execute(&mut self, action: &Self::Action, player: Self::Player) -> Self::Fitness {
        self.make_move(action.0);
        match self.check_winner() {
            None | Some(Winner::Tie) => Fitness::Even,
            Some(Winner::O) => {
                if player == Player::O {
                    Fitness::Win
                }
                else {
                    Fitness::Loss
                }
            }
            Some(Winner::X) => {
                if player == Player::X {
                    Fitness::Win
                }
                else {
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
