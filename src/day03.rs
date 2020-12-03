const INPUT: &str = include_str!("input/3.txt");

fn parse(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    other => panic!("Unrecognized character '{}'", other),
                })
                .collect::<Vec<bool>>()
        })
        .collect()
}

fn count_collisions(grid: &Vec<Vec<bool>>, dx: usize, dy: usize) -> usize {
    let mut x = 0;
    let mut y = 0;

    let mut collisions = 0;

    while y < grid.len() {
        if grid[y][x] {
            collisions += 1;
        }

        x = (x + dx) % grid[y].len();
        y += dy;
    }

    collisions
}

fn run_3a_with_input(input: &str) -> usize {
    let grid = parse(input);

    count_collisions(&grid, 3, 1)
}

pub fn run_3a() -> usize {
    run_3a_with_input(INPUT)
}

fn run_3b_with_input(input: &str) -> usize {
    let grid = parse(input);

    let a = count_collisions(&grid, 1, 1);
    let b = count_collisions(&grid, 3, 1);
    let c = count_collisions(&grid, 5, 1);
    let d = count_collisions(&grid, 7, 1);
    let e = count_collisions(&grid, 1, 2);

    a * b * c * d * e
}

pub fn run_3b() -> usize {
    run_3b_with_input(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_3: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn sample_3a() {
        assert_eq!(run_3a_with_input(SAMPLE_3), 7);
    }

    #[test]
    fn sample_3b() {
        assert_eq!(run_3b_with_input(SAMPLE_3), 2 * 7 * 3 * 4 * 2)
    }
}
