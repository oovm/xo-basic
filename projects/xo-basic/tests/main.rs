use rubot::{Bot, Logger, ToCompletion};
use xo_basic::{Action, Game, Player};

#[test]
fn man_first() {
    let mut game = Game::default();
    game.make_move(0);
    let mut opponent = Bot::new(Player::O);
    let mut logger = Logger::new(ToCompletion);
    assert_eq!(opponent.select(&game, &mut logger).unwrap(), Action::from(1));
}

#[test]
fn bot_first() {
    let mut game = Game::default();
    let mut opponent = Bot::new(Player::X);
    let mut logger = Logger::new(ToCompletion);
    assert_eq!(opponent.select(&game, &mut logger).unwrap(), Action::from(0));
}
