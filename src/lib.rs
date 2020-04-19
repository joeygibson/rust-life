use std::thread::sleep;
use std::time::Duration;

use easycurses::*;

use crate::entities::board::{new_board, Board};

mod entities;

pub fn setup_screen() -> EasyCurses {
    let mut screen = EasyCurses::initialize_system().unwrap();

    screen.set_cursor_visibility(CursorVisibility::Invisible);
    screen.set_echo(false);

    screen
}

pub fn play(
    screen: &mut EasyCurses,
    columns: Option<i32>,
    rows: Option<i32>,
    iterations: Option<u32>,
    hacker: bool,
    wait: Option<u64>,
) {
    let (default_rows, default_columns) = screen.get_row_col_count();

    let rows = match rows {
        None => default_rows,
        Some(r) => r,
    };

    let columns = match columns {
        None => default_columns,
        Some(c) => c,
    };

    let wait = match wait {
        None => Duration::from_millis(500),
        Some(w) => Duration::from_millis(w),
    };

    let mut board = new_board(rows, columns, hacker);

    match iterations {
        None => loop {
            board.step();
            display_board(&board);

            sleep(wait);
        },
        Some(iterations) => {
            for _ in 0..iterations {
                board.step();
                display_board(&board);

                sleep(wait);
            }
        }
    }

    screen.refresh();

    screen.get_input();
}

fn display_board(board: &Board) {}
