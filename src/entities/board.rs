use rand::{thread_rng, Rng};

use crate::entities::cell::{new_cell, Cell};

#[derive(Debug)]
pub struct Board {
    rows: i32,
    columns: i32,
    cells: Vec<Vec<Cell>>,
}

pub fn new_board(rows: i32, columns: i32, hacker: bool) -> Board {
    let cells = create_cells(rows, columns);

    let mut board = Board {
        rows,
        columns,
        cells,
    };

    if hacker {
        board.hacker_seed();
    } else {
        board.random_seed();
    }

    board
}

impl Board {
    fn hacker_seed(&mut self) {
        self.cells[0][2].alive = true;
        self.cells[1][0].alive = true;
        self.cells[1][2].alive = true;
        self.cells[2][1].alive = true;
        self.cells[2][2].alive = true;
    }

    fn random_seed(&mut self) {
        let mut rng = thread_rng();

        let times = rng.gen_range(0, self.rows * self.columns);

        for _ in 0..times {
            let row_num = rng.gen_range(0, self.rows);
            let col_num = rng.gen_range(0, self.columns);

            self.cells[row_num as usize][col_num as usize].alive = true;
        }
    }

    pub fn step(&mut self) {
        let local_cells = &mut self.cells;

        for (ri, row) in local_cells.iter_mut().enumerate() {
            for (ci, cell) in row.iter_mut().enumerate() {
                let neighbors = get_neighbors(local_cells, self.rows, self.columns, ri, ci);

                cell.step(neighbors);
            }
        }
    }
}

fn get_neighbors(
    cells: &Vec<Vec<Cell>>,
    rows: i32,
    columns: i32,
    r: usize,
    c: usize,
) -> Vec<&Cell> {
    let mut neighbors: Vec<&Cell> = vec![];

    for i in (-1 as i32)..2 {
        for j in (-1 as i32)..2 {
            let mut new_r: i32 = r as i32 + i;
            let mut new_c: i32 = c as i32 + j;

            if new_r >= rows {
                new_r = 0;
            } else if new_r < 0 {
                new_r = rows - 1;
            }

            if new_c >= columns {
                new_c = 0;
            } else if new_c < 0 {
                new_c = columns - 1;
            }

            if new_r != (r as i32) && new_c != (c as i32) {
                neighbors.push(&cells[new_r as usize][new_c as usize]);
            }
        }
    }

    neighbors
}

fn create_cells(rows: i32, columns: i32) -> Vec<Vec<Cell>> {
    let mut cells: Vec<Vec<Cell>> = vec![];

    for _ in 0..rows {
        let mut row: Vec<Cell> = vec![];

        for _ in 0..columns {
            let cell = new_cell();
            row.push(cell);
        }

        cells.push(row);
    }

    cells
}
