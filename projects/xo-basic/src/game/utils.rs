#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    None,
    X,
    O,
}

impl Default for Tile {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
    X,
    O,
}

impl Iterator for Player {
    type Item = Player;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Player::X => Some(Player::O),
            Player::O => Some(Player::X),
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::X
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Winner {
    X,
    O,
    Tie,
}
