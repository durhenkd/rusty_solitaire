mod utils;

use crate::utils::get_game_data;
use solitaire::{CardValue, GameAction, GameData};

#[test]
fn game_creation() {
    let game = GameData::new(10000);

    assert_eq!(game.tables.playing_table[0].len(), 1);
    assert_eq!(game.tables.playing_table[1].len(), 2);
    assert_eq!(game.tables.playing_table[2].len(), 3);
    assert_eq!(game.tables.playing_table[3].len(), 4);
    assert_eq!(game.tables.playing_table[4].len(), 5);
    assert_eq!(game.tables.playing_table[5].len(), 6);
    assert_eq!(game.tables.playing_table[6].len(), 7);

    assert_eq!(game.tables.foundation_table[0].len(), 0);
    assert_eq!(game.tables.foundation_table[1].len(), 0);
    assert_eq!(game.tables.foundation_table[2].len(), 0);
    assert_eq!(game.tables.foundation_table[3].len(), 0);

    assert_eq!(game.tables.drawn_table.len(), 0);
    assert_eq!(game.tables.extra_table.len(), 24);

    assert!(game.tables.playing_table[0][0].is_face_up);
    assert!(game.tables.playing_table[1][1].is_face_up);
    assert!(game.tables.playing_table[2][2].is_face_up);
    assert!(game.tables.playing_table[3][3].is_face_up);
    assert!(game.tables.playing_table[4][4].is_face_up);
    assert!(game.tables.playing_table[5][5].is_face_up);
    assert!(game.tables.playing_table[6][6].is_face_up);

    assert!(!game.tables.playing_table[1][0].is_face_up);
    assert!(!game.tables.playing_table[2][1].is_face_up);
    assert!(!game.tables.playing_table[3][2].is_face_up);
    assert!(!game.tables.playing_table[4][3].is_face_up);
    assert!(!game.tables.playing_table[5][4].is_face_up);
    assert!(!game.tables.playing_table[6][5].is_face_up);
}

#[test]
fn p2p_moves() {
    let mut game = get_game_data();

    game.do_(GameAction::MoveToPlayingFromPlaying(0, 1, 1, 1))
        .expect("This should work");
    assert_eq!(game.tables.playing_table[1][1].value, CardValue::Ace);

    game.do_(GameAction::MoveToPlayingFromPlaying(2, 0, 3, 0))
        .expect("This should work");
    assert_eq!(game.tables.playing_table[3][0].value, CardValue::King);

    game.do_(GameAction::MoveToPlayingFromPlaying(4, 0, 3, 1))
        .expect("This should work");
    assert_eq!(game.tables.playing_table[4].len(), 0);
    assert_eq!(game.tables.playing_table[3].len(), 4);
}

#[test]
#[should_panic]
fn p2p_illegal_follow() {
    let mut game = get_game_data();
    game.do_(GameAction::MoveToPlayingFromPlaying(0, 1, 2, 1))
        .expect("This should not work!");
}

#[test]
#[should_panic]
fn p2p_illegal_put_on_empty_stack() {
    let mut game = get_game_data();
    game.do_(GameAction::MoveToPlayingFromPlaying(0, 1, 4, 0))
        .expect("This should not work!");
}

#[test]
fn draw_move_stack_sizes() {
    let mut game = get_game_data();

    assert_eq!(game.tables.extra_table.len(), 3);
    game.do_(GameAction::DrawCard).expect("This should work");
    assert_eq!(game.tables.extra_table.len(), 2);
    assert_eq!(game.tables.drawn_table.len(), 1);
    game.do_(GameAction::DrawCard).expect("This should work");
    game.do_(GameAction::DrawCard).expect("This should work");
    assert_eq!(game.tables.extra_table.len(), 0);
    assert_eq!(game.tables.drawn_table.len(), 3);
    game.do_(GameAction::DrawCard).expect("This should work");
    assert_eq!(game.tables.extra_table.len(), 3);
}

#[test]
fn draw_move_cards() {
    let mut game = get_game_data();

    assert_eq!(game.tables.extra_table[2].value, CardValue::Five);
    game.do_(GameAction::DrawCard).expect("This should work");
    assert_eq!(game.tables.extra_table[1].value, CardValue::Seven);
    assert_eq!(game.tables.drawn_table[0].value, CardValue::Five);
    game.do_(GameAction::DrawCard).expect("This should work");
    game.do_(GameAction::DrawCard).expect("This should work");
    game.do_(GameAction::DrawCard).expect("This should work");
    assert_eq!(game.tables.extra_table[2].value, CardValue::Five);
}

#[test]
fn p2f_moves() {
    let mut game = get_game_data();

    game.do_(GameAction::MoveToFoundationFromPlaying(4, 0))
        .expect("This should work");
    assert_eq!(game.tables.foundation_table[0][1].value, CardValue::Ten);
    game.do_(GameAction::MoveToFoundationFromPlaying(0, 1))
        .expect("This should work");
    assert_eq!(game.tables.foundation_table[1][0].value, CardValue::Ace);
}

#[test]
#[should_panic]
fn p2f_illegal_moves() {
    let mut game = get_game_data();

    game.do_(GameAction::MoveToFoundationFromPlaying(4, 1))
        .expect("This should not work");
}

#[test]
#[should_panic]
fn p2f_illegal_move_king_to_empty() {
    let mut game = get_game_data();

    game.do_(GameAction::MoveToFoundationFromPlaying(2, 1))
        .expect("This should not work");
}

#[test]
fn e2p_move() {
    let mut game = get_game_data();

    game.do_(GameAction::DrawCard).expect("This should work");
    game.do_(GameAction::DrawCard).expect("This should work");

    game.do_(GameAction::MoveToPlayingFromDrawn(5))
        .expect("This should work");

    assert_eq!(game.tables.drawn_table.len(), 1);
    assert_eq!(game.tables.extra_table.len(), 1);
    assert_eq!(game.tables.playing_table[5].len(), 2);
}

#[test]
#[should_panic]
fn e2p_illegal_move() {
    let mut game = get_game_data();

    game.do_(GameAction::DrawCard).expect("This should work");

    game.do_(GameAction::MoveToPlayingFromDrawn(5))
        .expect("This should work");
}

#[test]
#[should_panic]
fn e2p_illegal_move_empty() {
    let mut game = get_game_data();

    game.do_(GameAction::MoveToPlayingFromDrawn(5))
        .expect("This should work");
}
