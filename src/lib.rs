use std::thread::sleep;
use std::time::Duration;

use easycurses::*;
use easycurses::Color::Green;
use easycurses::Color::Black;
use easycurses::constants::acs;

use crate::entities::board::{new_board, Board};

mod entities;

pub fn setup_screen() -> EasyCurses {
    let mut screen = EasyCurses::initialize_system().unwrap();

    screen.set_cursor_visibility(CursorVisibility::Invisible);
    screen.set_echo(false);
    screen.set_color_pair(colorpair!(Green on Black));
    screen.set_input_mode(InputMode::Character);
    screen.set_input_timeout(TimeoutMode::Immediate);
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

    let mut board = new_board(rows - 2, columns - 2, hacker);

    display_board(screen, &board);
    screen.refresh();

    match iterations {
        None => loop {
            board.step();
            display_board(screen, &board);
            screen.refresh();

            if let Some(c) = screen.get_input() {
                break;
            }

            sleep(wait);
        },
        Some(iterations) => {
            for _ in 0..iterations {
                board.step();
                display_board(screen, &board);
                screen.refresh();

                if let Some(c) = screen.get_input() {
                    break;
                }
                
                sleep(wait);
            }
        }
    }

    screen.refresh();
}

fn display_board(screen: &mut EasyCurses, board: &Board) {
    screen.move_rc(0, 0);
    screen.insert_char(acs::ulcorner());

    for j in 0..board.columns {
        screen.move_rc(0, j + 1);
        screen.print_char(acs::hline());
    }

    screen.move_rc(0, board.columns + 1);
    screen.print_char(acs::urcorner());

    for (ri, row) in board.cells.iter().enumerate() {
        screen.move_rc((ri + 1) as i32, 0);
        screen.print_char(acs::vline());

        for (ci, cell) in row.iter().enumerate() {
            screen.move_rc((ri + 1) as i32, (ci + 1) as i32);
            screen.print_char(cell.to_printable_char());
        }

        screen.move_rc((ri + 1) as i32, board.columns + 1);
        screen.print_char(acs::vline());
    }

    screen.move_rc(board.rows + 1, 0);
    screen.print_char(acs::llcorner());

    for j in 0..board.columns {
        screen.move_rc(board.rows + 1, j + 1);
        screen.print_char(acs::hline());
    }

    screen.move_rc(board.rows + 1, board.columns + 1);
    screen.print_char(acs::lrcorner());
}
