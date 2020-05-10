use rand::{thread_rng, Rng};

use crate::entities::cell::{new_cell, Cell};

#[derive(Debug)]
pub struct Board {
    pub rows: i32,
    pub columns: i32,
    pub cells: Vec<Vec<Cell>>,
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
        self.cells[0][1].alive = true;
        self.cells[1][2].alive = true;
        self.cells[2][0].alive = true;
        self.cells[2][1].alive = true;
        self.cells[2][2].alive = true;
    }

    fn random_seed(&mut self) {
        let mut rng = thread_rng();

        let times = rng.gen_range(1, self.rows * self.columns);

        for _ in 0..times {
            let row_num = rng.gen_range(0, self.rows);
            let col_num = rng.gen_range(0, self.columns);

            self.cells[row_num as usize][col_num as usize].alive = true;
        }
    }

    pub fn step(&mut self) {
        let mut new_cells = create_cells(self.rows, self.columns);

        for (ri, row) in self.cells.iter().enumerate() {
            for (ci, cell) in row.iter().enumerate() {
                let neighbors = self.get_neighbors(  ri, ci);

                new_cells[ri][ci] = cell.step(neighbors);
            }
        }

        self.cells = new_cells;
    }

    fn get_neighbors(
        &self,
        r: usize,
        c: usize,
    ) -> Vec<&Cell> {
        let mut neighbors: Vec<&Cell> = vec![];

        for i in -1..2 {
            for j in -1..2 {
                let mut new_r: i32 = r as i32 + i;
                let mut new_c: i32 = c as i32 + j;

                if new_r >= self.rows {
                    new_r = 0;
                } else if new_r < 0 {
                    new_r = self.rows - 1;
                }

                if new_c >= self.columns {
                    new_c = 0;
                } else if new_c < 0 {
                    new_c = self.columns - 1;
                }

                if !(new_r == (r as i32) && new_c == (c as i32)) {
                    neighbors.push(&self.cells[new_r as usize][new_c as usize]);
                }
            }
        }

        neighbors
    }
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
