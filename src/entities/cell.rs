#[derive(Debug)]
pub struct Cell {
    pub alive: bool,
}

impl Cell {
    pub fn step(&mut self, neighbors: Vec<&Cell>) {
        let live_count = neighbors.iter().filter(|n| n.alive).count();

        if self.alive {
            if live_count < 2 || live_count > 3 {
                self.alive = false;
            }
        } else {
            if !self.alive && live_count == 3 {
                self.alive = true;
            }
        }
    }
}

pub fn new_cell() -> Cell {
    Cell { alive: false }
}
