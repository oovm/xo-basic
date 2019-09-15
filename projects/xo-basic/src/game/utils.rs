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

impl Player {
    pub fn next(&mut self) {
        *self = self.switch()
    }
    pub fn switch(&self) -> Player {
        use Player::{O, X};
        match self {
            X => O,
            O => X,
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
    Draw,
    X,
    O,
}
