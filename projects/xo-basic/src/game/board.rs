use crate::game::utils::{Tile, Winner};

#[derive(Default, Debug)]
pub struct Board {
    state: [Tile; 9],
}

impl Board {
    fn match_winner(&self, tile: &Tile) -> Option<Winner> {
        match tile {
            Tile::X => Some(Winner::X),
            Tile::O => Some(Winner::O),
            Tile::None => None,
        }
    }
    pub fn check_winner(&self) -> Option<Winner> {
        let s = &self.state;
        // check row 1
        if s[0] != Tile::None && s[0] == s[1] && s[1] == s[2] {
            self.match_winner(&s[0])
        }
        // check row 2
        else if s[3] != Tile::None && s[3] == s[4] && s[3] == s[5] {
            None
        }
        // check row 3
        else if s[6] != Tile::None && s[6] == s[7] && s[6] == s[8] {
            None
        }
        // check column 1
        else if s[6] != Tile::None && s[6] == s[7] && s[6] == s[8] {
            None
        }
        // check column 2
        else if s[6] != Tile::None && s[6] == s[7] && s[6] == s[8] {
            None
        }
        // check column 3
        else if s[6] != Tile::None && s[6] == s[7] && s[6] == s[8] {
            None
        }
        // check diag 1
        else if s[6] != Tile::None && s[6] == s[7] && s[6] == s[8] {
            None
        }
        // check diag 2
        else if s[6] != Tile::None && s[6] == s[7] && s[6] == s[8] {
            None
        } else {
            None
        }
    }


}

#[test]
fn test() {
    // let a: &[Tile; 9] = &[Tile::None,Tile::None,Tile::None,Tile::None,Tile::None,Tile::None,Tile::None,Tile::None,Tile::None];
    println!("{:?}", Board::default())
}
