const BOARD_SIZE: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Piece {
    X,
    O,
}

impl Piece {
    pub fn other(self) -> Piece {
        match self {
            Piece::X => Piece::O,
            Piece::O => Piece::X,
        }
    }
}

pub type Tile = Option<Piece>;
pub type Tiles = [[Tile; BOARD_SIZE]; BOARD_SIZE];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Winner {
    X,
    O,
    Tie,
}

#[derive(Debug, Clone)]
pub enum MoveError {
    GameAlreadyOver,
    InvalidPosition { row: usize, col: usize },
    TileNotEmpty { other_piece: Piece, row: usize, col: usize },
}

#[derive(Debug, Clone)]
pub struct Game {
    tiles: Tiles,
    current_piece: Piece,
    winner: Option<Winner>,
}

impl Game {
    pub fn new() -> Self {
        Self { tiles: Default::default(), current_piece: Piece::X, winner: None }
    }

    pub fn make_move(&mut self, row: usize, col: usize) -> Result<(), MoveError> {
        if self.is_finished() {
            return Err(MoveError::GameAlreadyOver);
        }
        else if row >= self.tiles.len() || col >= self.tiles[0].len() {
            return Err(MoveError::InvalidPosition { row, col });
        }
        else if let Some(other_piece) = self.tiles[row][col] {
            return Err(MoveError::TileNotEmpty { other_piece, row, col });
        }

        self.tiles[row][col] = Some(self.current_piece);
        self.current_piece = self.current_piece.other();
        self.update_winner(row, col);
        Ok(())
    }

    fn update_winner(&mut self, row: usize, col: usize) {
        let rows = self.tiles.len();
        let cols = self.tiles[0].len();

        let tiles_row = self.tiles[row];

        let tiles_col = [self.tiles[0][col], self.tiles[1][col], self.tiles[2][col]];

        assert!(rows == 3 && cols == 3, "This code was written with the assumption that there are three rows and columns");
        let tiles_diagonal_1 =
            if row == col { [self.tiles[0][0], self.tiles[1][1], self.tiles[2][2]] } else { [None, None, None] };

        let tiles_diagonal_2 =
            if (rows - row - 1) == col { [self.tiles[0][2], self.tiles[1][1], self.tiles[2][0]] } else { [None, None, None] };

        fn check_winner(row: &[Tile]) -> Option<Winner> {
            if row[0] == row[1] && row[1] == row[2] {
                match row[0] {
                    Some(Piece::X) => Some(Winner::X),
                    Some(Piece::O) => Some(Winner::O),
                    None => None,
                }
            }
            else {
                None
            }
        }
        self.winner = self
            .winner
            .or_else(|| check_winner(&tiles_row))
            .or_else(|| check_winner(&tiles_col))
            .or_else(|| check_winner(&tiles_diagonal_1))
            .or_else(|| check_winner(&tiles_diagonal_2));

        self.winner = self.winner.or_else(|| {
            if self.tiles.iter().all(|row| row.iter().all(|tile| tile.is_some())) { Some(Winner::Tie) } else { None }
        });
    }

    pub fn is_finished(&self) -> bool {
        self.winner.is_some()
    }

    pub fn winner(&self) -> Option<Winner> {
        self.winner
    }

    pub fn current_piece(&self) -> Piece {
        self.current_piece
    }

    pub fn tiles(&self) -> &Tiles {
        &self.tiles
    }
}
