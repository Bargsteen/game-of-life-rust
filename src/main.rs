use std::fmt;

fn main() {
    let mut grid = Grid::new(&Dimensions {
        rows: 10usize,
        cols: 10usize,
    });
    grid.grid[5][5].reproduce();
    grid.grid[5][6].reproduce();
    grid.grid[5][7].reproduce();

    print!("{}", grid);
    for i in 0..10 {
        grid.tick();
        print!("{}", grid);
    }
}

#[derive(Clone, PartialEq)]
enum State {
    Dead,
    Alive,
}

#[derive(Clone)]
struct Cell {
    state: State,
}

#[derive(Clone)]
struct Dimensions {
    rows: usize,
    cols: usize,
}

struct Grid {
    dimensions: Dimensions,
    grid: Vec<Vec<Cell>>,
}

impl Cell {
    fn kill(&mut self) {
        self.state = State::Dead;
    }

    fn reproduce(&mut self) {
        self.state = State::Alive;
    }

    fn isAlive(&self) -> bool {
        self.state == State::Alive
    }
}

impl Grid {
    pub fn new(dim: &Dimensions) -> Grid {
        Grid {
            dimensions: dim.clone(),
            grid: vec![vec![Cell { state: State::Dead }; dim.rows]; dim.cols],
        }
    }

    pub fn tick(&mut self) {
        for row in 1..self.dimensions.rows - 1 {
            for col in 1..self.dimensions.cols - 1 {
                let neighbours = self.get_neighbours_of(&row, &col);
                let alive_neighbour_count = neighbours.iter().fold(0, |sum, c| {
                    if c.state == State::Alive {
                        sum + 1
                    } else {
                        sum
                    }
                });

                match (&self.grid[row][col].state, alive_neighbour_count) {
                    (State::Alive, 0...1) => self.grid[row][col].kill(),
                    (State::Alive, 4...8) => self.grid[row][col].kill(),
                    (State::Dead, 3) => self.grid[row][col].reproduce(),
                    (_, _) => (),
                };
            }
        }
    }

    fn get_neighbours_of(&self, row: &usize, col: &usize) -> Vec<&Cell> {
        let row = *row;
        let col = *col;

        if row == 0usize
            || col == 0usize
            || row >= self.dimensions.rows - 1
            || col >= self.dimensions.cols - 1
        {
            panic!("Cannot get neighbours of edges");
        }

        vec![
            &self.grid[row - 1][col - 1],
            &self.grid[row - 1][col],
            &self.grid[row - 1][col + 1],
            &self.grid[row][col - 1],
            &self.grid[row][col + 1],
            &self.grid[row + 1][col - 1],
            &self.grid[row + 1][col],
            &self.grid[row + 1][col + 1],
        ]
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Grid of size ({}, {})\n ----",
            self.dimensions.rows, self.dimensions.cols
        );

        for row in 0..self.dimensions.rows {
            for col in 0..self.dimensions.cols {
                let s = if self.grid[row][col].isAlive() { 1 } else { 0 };
                write!(f, "{} ", s);
            }
            writeln!(f);
        }
        writeln!(f, "------")
    }
}