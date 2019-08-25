mod ai;
mod game;

use std::io::{self, Write};
use std::process;
use std::time::Duration;





use game::{Game, MoveError, Piece, Tiles, Winner};

#[derive(Debug, Clone)]
pub struct InvalidMove(String);

fn prompt_move() -> (usize, usize) {
    loop {
        print!("Enter move (e.g. A1): ");
        io::stdout().flush().expect("Failed to flush stdout");
        let line = read_line();
        match parse_move(&line) {
            Ok((row, col)) => break (row, col),
            Err(InvalidMove(invalid_str)) => {
                eprintln!("Invalid move: '{}'. Please try again.", invalid_str,)
            }
        }
    }
}

fn parse_move(input: &str) -> Result<(usize, usize), InvalidMove> {
    if input.len() != 2 {
        return Err(InvalidMove(input.to_string()));
    }

    let col = match &input[0..1] {
        "A" | "a" => 0,
        "B" | "b" => 1,
        "C" | "c" => 2,
        invalid => return Err(InvalidMove(invalid.to_string())),
    };

    let row = match &input[1..2] {
        "1" => 0,
        "2" => 1,
        "3" => 2,
        _ => return Err(InvalidMove(input.to_string())),
    };

    Ok((row, col))
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    if input.is_empty() {
        println!();
        process::exit(0);
    }
    let len_without_newline = input.trim_end().len();
    input.truncate(len_without_newline);
    input
}

fn print_tiles(tiles: &Tiles) {
    print!("  ");
    for j in 0..tiles[0].len() as u8 {
        print!(" {}", (b'A' + j) as char);
    }
    println!();

    for (i, row) in tiles.iter().enumerate() {
        print!(" {}", i + 1);
        for tile in row {
            print!(
                " {}",
                match *tile {
                    Some(Piece::X) => "x",
                    Some(Piece::O) => "o",
                    None => "\u{25A2}",
                }
            );
        }
        println!();
    }

    println!();
}

fn main() {
    use rubot::Bot;

    let mut game = Game::new();
    let mut opponent = Bot::new(Piece::O);
    while !game.is_finished() {
        print_tiles(game.tiles());
        match game.current_piece() {
            Piece::X => {
                println!("Current piece: x");
                let (row, col) = prompt_move();

                match game.make_move(row, col) {
                    Ok(()) => {}
                    Err(MoveError::GameAlreadyOver) => {
                        unreachable!("Game was already over when it should not have been")
                    }
                    Err(MoveError::InvalidPosition { row, col }) => unreachable!(
                        "Should not be able to enter an invalid move, but still got ({}, {})",
                        row, col
                    ),
                    Err(MoveError::TileNotEmpty {
                            other_piece,
                            row,
                            col,
                        }) => eprintln!(
                        "The tile at position {}{} already has piece {} in it!",
                        row + 1,
                        (b'A' + col as u8) as char,
                        match other_piece {
                            Piece::X => "x",
                            Piece::O => "o",
                        },
                    ),
                }
            }
            Piece::O => {
                let action = opponent.select(&game, Duration::from_secs(1)).unwrap();
                game.make_move(action.0, action.1).unwrap();
            }
        }
    }
    print_tiles(game.tiles());
    match game.winner().expect("finished game should have winner") {
        Winner::X => println!("x wins!"),
        Winner::O => println!("o wins!"),
        Winner::Tie => println!("Tie!"),
    }
}