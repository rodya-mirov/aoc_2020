const INPUT: &str = include_str!("input/11.txt");

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum State {
    Empty,
    Occupied,
    Floor,
}

type Grid = Vec<Vec<State>>;

fn is_occupied(grid: &Grid, x: i32, y: i32) -> bool {
    if y < 0 || y >= (grid.len() as i32) {
        false
    } else {
        let row = &grid[y as usize];
        if x < 0 || x >= (row.len() as i32) {
            false
        } else {
            row[x as usize] == State::Occupied
        }
    }
}

fn is_occupied_dir(grid: &Grid, mut x: i32, mut y: i32, dx: i32, dy: i32) -> bool {
    loop {
        x += dx;
        y += dy;
        if y < 0 || y >= (grid.len() as i32) {
            return false;
        } else {
            let row = &grid[y as usize];
            if x < 0 || x >= (row.len() as i32) {
                return false;
            } else {
                let state = row[x as usize];
                match state {
                    // Floor is a no-op, keep looking
                    State::Floor => {}
                    State::Empty => {
                        return false;
                    }
                    State::Occupied => {
                        return true;
                    }
                }
            }
        }
    }
}

fn next_a(grid: &Grid) -> Grid {
    let num_cols = grid[0].len() as i32;
    let num_rows = grid.len() as i32;

    let mut next_grid = grid.clone();

    for y in 0..num_rows {
        for x in 0..num_cols {
            let adj_occ: usize = [
                (x - 1, y - 1),
                (x - 1, y),
                (x - 1, y + 1),
                (x, y - 1),
                (x, y + 1),
                (x + 1, y - 1),
                (x + 1, y),
                (x + 1, y + 1),
            ]
            .iter()
            .copied()
            .filter(|(adj_x, adj_y)| is_occupied(grid, *adj_x, *adj_y))
            .count();

            let next_state = match grid[y as usize][x as usize] {
                State::Occupied => {
                    if adj_occ >= 4 {
                        State::Empty
                    } else {
                        State::Occupied
                    }
                }
                State::Empty => {
                    if adj_occ == 0 {
                        State::Occupied
                    } else {
                        State::Empty
                    }
                }
                State::Floor => State::Floor,
            };
            next_grid[y as usize][x as usize] = next_state;
        }
    }

    next_grid
}

fn next_b(grid: &Grid) -> Grid {
    let num_cols = grid[0].len() as i32;
    let num_rows = grid.len() as i32;

    let mut next_grid = grid.clone();

    for y in 0..num_rows {
        for x in 0..num_cols {
            let adj_occ: usize = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ]
            .iter()
            .copied()
            .filter(|(adj_x, adj_y)| is_occupied_dir(grid, x, y, *adj_x, *adj_y))
            .count();

            let next_state = match grid[y as usize][x as usize] {
                State::Occupied => {
                    if adj_occ >= 5 {
                        State::Empty
                    } else {
                        State::Occupied
                    }
                }
                State::Empty => {
                    if adj_occ == 0 {
                        State::Occupied
                    } else {
                        State::Empty
                    }
                }
                State::Floor => State::Floor,
            };
            next_grid[y as usize][x as usize] = next_state;
        }
    }

    next_grid
}

fn parse_grid(input: &str) -> Grid {
    let mut max_len = 0;
    let mut grid = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();

        for c in line.chars() {
            let state = match c {
                'L' => State::Empty,
                '#' => State::Occupied,
                '.' => State::Floor,
                other => panic!("Unrecognized state char {}", other),
            };
            row.push(state);
        }

        max_len = max_len.max(row.len());
        grid.push(row);
    }

    for (i, row) in grid.iter().enumerate() {
        if row.len() != max_len {
            panic!(
                "Row {} has length {}, but max should be {}",
                i,
                row.len(),
                max_len
            );
        }
    }

    grid
}

fn count_occupied(grid: &Grid) -> usize {
    grid.iter()
        .map(|row| {
            row.iter()
                .copied()
                .filter(|&s| s == State::Occupied)
                .count()
        })
        .sum()
}

fn run_11a_with_input(input: &str) -> usize {
    let mut grid = parse_grid(input);

    loop {
        let next_grid = next_a(&grid);
        if next_grid == grid {
            return count_occupied(&grid);
        }
        grid = next_grid;
    }
}

fn run_11b_with_input(input: &str) -> usize {
    let mut grid = parse_grid(input);

    loop {
        let next_grid = next_b(&grid);
        if next_grid == grid {
            return count_occupied(&grid);
        }
        grid = next_grid;
    }
}

pub fn run_11a() -> usize {
    run_11a_with_input(INPUT)
}

pub fn run_11b() -> usize {
    run_11b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn sample_11a() {
        assert_eq!(run_11a_with_input(SAMPLE_INPUT), 37);
    }

    #[test]
    fn sample_11b() {
        assert_eq!(run_11b_with_input(SAMPLE_INPUT), 26);
    }
}
