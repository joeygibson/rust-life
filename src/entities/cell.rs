#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub alive: bool,
}

impl Cell {
    pub fn step(&self, neighbors: Vec<&Cell>) -> Cell {
        let live_count = neighbors.iter().filter(|n| n.alive).count();

        let mut new_cell = self.clone();

        if self.alive {
            if live_count < 2 || live_count > 3 {
                new_cell.alive = false;
            }
        } else {
            if !self.alive && live_count == 3 {
                new_cell.alive = true;
            }
        }

        new_cell
    }

    pub fn to_printable_char(&self) -> char {
        if self.alive {
            '*'
        } else {
            ' '
        }
    }
}

pub fn new_cell() -> Cell {
    Cell { alive: false }
}
