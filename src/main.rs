use crossterm::{
    cursor::{Hide, MoveToPreviousLine},
    event::{poll, read, Event, KeyCode},
    execute,
    style::{Print, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use solitaire::{Card, GameAction, GameData, GameTables};
use std::{
    cmp::{max, min},
    io::stdout,
    time::Duration,
};

fn get_last_card_to_string(cards: &Vec<Card>, highlighted: bool) -> String {
    if cards.len() == 0 {
        get_card_nr_to_string(cards, 0, highlighted)
    } else {
        get_card_nr_to_string(cards, cards.len() - 1, highlighted)
    }
}

fn get_card_nr_to_string(cards: &Vec<Card>, i: usize, highlighted: bool) -> String {
    if cards.len() <= i {
        if highlighted {
            return String::from("███");
        } else {
            return String::from("   ");
        }
    }

    if highlighted {
        cards[i].to_string().on_white().black().to_string()
    } else {
        cards[i].to_string()
    }
}

fn to_string(tables: &GameTables, x: usize, y: usize) -> String {
    let header = format!(
        "┌───┬───┬───┬───┐   ┌───┬───┐
    \r│{}│{}│{}│{}│   │{}│{}│
    \r└───┴───┴───┴───┘   └───┴───┘",
        get_last_card_to_string(&tables.foundation_table[0], y == 0 && x == 0),
        get_last_card_to_string(&tables.foundation_table[1], y == 0 && x == 1),
        get_last_card_to_string(&tables.foundation_table[2], y == 0 && x == 2),
        get_last_card_to_string(&tables.foundation_table[3], y == 0 && x == 3),
        get_last_card_to_string(&tables.drawn_table, y == 0 && x == 5),
        get_last_card_to_string(&tables.extra_table, y == 0 && x == 6)
    );

    let mut body = String::new();

    for i in 0..=12 {
        let body_line = format!(
            " {} {} {} {} {} {} {}\n",
            get_card_nr_to_string(&tables.playing_table[0], i, y == i + 1 && x == 0),
            get_card_nr_to_string(&tables.playing_table[1], i, y == i + 1 && x == 1),
            get_card_nr_to_string(&tables.playing_table[2], i, y == i + 1 && x == 2),
            get_card_nr_to_string(&tables.playing_table[3], i, y == i + 1 && x == 3),
            get_card_nr_to_string(&tables.playing_table[4], i, y == i + 1 && x == 4),
            get_card_nr_to_string(&tables.playing_table[5], i, y == i + 1 && x == 5),
            get_card_nr_to_string(&tables.playing_table[6], i, y == i + 1 && x == 6),
        );

        body.push_str(&body_line);
    }

    format!("{}\n\n{}", header, body)
}

fn display(table: String) {
    execute!(stdout(), Hide, MoveToPreviousLine(17), Print(table),).unwrap();
}

fn process_command(data: &mut GameData, x: usize, y: usize) {
    match (x, y) {
        (4, 0) => return, // the empty spot just return
        (5, 0) => {
            // on the draw stack
            for stack_index in 0..4 {
                match data.do_(GameAction::MoveToFoundationFromDrawn(stack_index)) {
                    Ok(a) => {
                        data.action_history.push(a);
                        return;
                    }
                    Err(_) => (),
                }
            }
            for stack_index in 0..7 {
                match data.do_(GameAction::MoveToPlayingFromDrawn(stack_index)) {
                    Ok(a) => {
                        data.action_history.push(a);
                        return;
                    }
                    Err(_) => (),
                };
            }
        }
        (6, 0) => {
            // on the extra stack
            data.do_(GameAction::DrawCard).expect("This shouldn't fail");
            data.action_history.push(GameAction::DrawCard);
        }
        (_, 0) => (), // the the cursor is on the foundation
        (_, _) => {
            // the cursor is somewhere in the plaing table

            if data.tables.playing_table[x].len() < y {
                return;
            }

            if !data.tables.playing_table[x][y - 1].is_face_up {
                return;
            }

            if y == data.tables.playing_table[x].len() {
                for stack_index in 0..4 {
                    match data.do_(GameAction::MoveToFoundationFromPlaying(x, stack_index)) {
                        Ok(a) => {
                            data.action_history.push(a);
                            if let Ok(a) = data.do_(GameAction::ShowCard(x)) {
                                data.action_history.push(a);
                            }
                            return;
                        }
                        Err(_) => (),
                    }
                }
            }
            for stack_idex in 0..7 {
                match data.do_(GameAction::MoveToPlayingFromPlaying(
                    x,
                    y - 1,
                    stack_idex,
                    data.tables.playing_table[stack_idex].len(),
                )) {
                    Ok(a) => {
                        data.action_history.push(a);
                        if let Ok(a) = data.do_(GameAction::ShowCard(x)) {
                            data.action_history.push(a);
                        }
                        return;
                    }
                    Err(_) => (),
                }
            }
        }
    }
}
fn main() {
    let mut game = GameData::new(10000);

    let mut cursor_x: usize = 0;
    let mut cursor_y: usize = 0;
    print!("{}", to_string(&game.tables, cursor_x, cursor_y));
    display(to_string(&game.tables, cursor_x, cursor_y));

    loop {
        enable_raw_mode().unwrap();

        if !poll(Duration::from_millis(1_000)).unwrap() {
            continue;
        }

        let event = read().unwrap();

        disable_raw_mode().unwrap();

        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Up => {
                    cursor_y = max(0, usize::saturating_sub(cursor_y, 1));
                }
                KeyCode::Down => {
                    cursor_y = min(13, cursor_y + 1);
                }
                KeyCode::Left => {
                    cursor_x = max(0, usize::saturating_sub(cursor_x, 1));
                }
                KeyCode::Right => {
                    cursor_x = min(6, cursor_x + 1);
                }
                KeyCode::Char('c') => break,
                KeyCode::Enter => process_command(&mut game, cursor_x, cursor_y),
                KeyCode::Char('u') => game.undo(),
                _ => continue,
            }

            // TODO add a help menu at the bottom of the game
            // TODO add undo button

            display(to_string(&game.tables, cursor_x, cursor_y));
        }
    }
}
