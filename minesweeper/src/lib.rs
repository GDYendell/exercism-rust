use std::char::from_digit;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // Find mines
    let mine_coordinates = minefield
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.as_bytes()
                .iter()
                .enumerate()
                .filter_map(
                    move |(x, &cell)| if cell == b'*' { Some((y, x)) } else { None },
                )
        })
        .collect::<Vec<_>>();

    // Define function to count mines surrounding a given coordinate
    let count_adjacent_mines = |y: usize, x: usize| -> char {
        match mine_coordinates
            .iter()
            .map(|&(mine_y, mine_x)| {
                if y.abs_diff(mine_y) <= 1 && x.abs_diff(mine_x) <= 1 {
                    1
                } else {
                    0
                }
            })
            .sum::<u32>()
        {
            0 => ' ',
            count => from_digit(count, 10).unwrap(),
        }
    };

    // Annotate minefield
    (0..minefield.len())
        .map(|y| {
            (0..minefield[0].len())
                .map(|x| {
                    if mine_coordinates.contains(&(y, x)) {
                        '*'
                    } else {
                        count_adjacent_mines(y, x)
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
}
