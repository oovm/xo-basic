use crate::game::utils::{Player, Tile, Winner};

#[derive(Default, Debug, Clone)]
pub struct Game {
    pub board: [Tile; 9],
    pub player: Player,
    pub record: Vec<usize>,
}

impl Game {
    fn match_winner(&self, a: &Tile, b: &Tile, c: &Tile) -> Option<Winner> {
        use Tile::{O, X};
        match (a, b, c) {
            (X, X, X) => Some(Winner::X),
            (O, O, O) => Some(Winner::O),
            _ => None,
        }
    }
    #[rustfmt::skip]
    pub fn check_winner(&self) -> Option<Winner> {
        let s = &self.board;
        // check row 1
        if let Some(s) = self.match_winner(&s[0], &s[1], &s[2]) { return Some(s); }
        // check row 2
        if let Some(s) = self.match_winner(&s[3], &s[4], &s[5]) { return Some(s); }
        // check row 3
        if let Some(s) = self.match_winner(&s[6], &s[7], &s[8]) { return Some(s); }
        // check column 1
        if let Some(s) = self.match_winner(&s[0], &s[3], &s[6]) { return Some(s); }
        // check column 2
        if let Some(s) = self.match_winner(&s[1], &s[4], &s[7]) { return Some(s); }
        // check column 3
        if let Some(s) = self.match_winner(&s[2], &s[5], &s[8]) { return Some(s); }
        // check diag 1
        if let Some(s) = self.match_winner(&s[0], &s[4], &s[8]) { return Some(s); }
        // check diag 2
        if let Some(s) = self.match_winner(&s[2], &s[4], &s[6]) { return Some(s); }
        if self.available_moves().len() == 0 { Some(Winner::Draw) } else { None }
    }

    pub fn available_moves(&self) -> Vec<usize> {
        let mut out = Vec::with_capacity(9);
        for i in 0..=8 {
            if self.board[i] == Tile::None {
                out.push(i)
            }
        }
        return out;
    }
    pub fn is_finished(&self) -> bool {
        match self.check_winner() {
            Some(_) => true,
            None => false,
        }
    }

    pub fn make_move(&mut self, p: usize) {
        self.record.push(p);
        self.board[p] = match self.player {
            Player::X => Tile::X,
            Player::O => Tile::O,
        };
        self.player.next()
    }
}

#[test]
fn test() {
    // let a: &[Tile; 9] = &[Tile::None,Tile::None,Tile::None,Tile::None,Tile::None,Tile::None,Tile::None,Tile::None,Tile::None];
    println!("{:?}", Game::default())
}
