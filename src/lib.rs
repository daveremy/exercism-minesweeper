use std::char;

const MINE: char = '*';

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let left = |r, c| (r, c - 1);
    let upper_left = |r, c| (r - 1, c - 1);
    let above = |r, c| (r - 1, c);
    let upper_right = |r, c| (r - 1, c + 1);
    let right = |r, c| (r, c + 1);
    let lower_right = |r, c| (r + 1, c + 1);
    let below = |r, c| (r + 1, c);
    let lower_left = |r, c| (r + 1, c - 1);
    let locations: Vec<Box<dyn Fn(isize, isize) -> (isize, isize)>> = vec![
        Box::new(left),
        Box::new(upper_left),
        Box::new(above),
        Box::new(upper_right),
        Box::new(right),
        Box::new(lower_right),
        Box::new(below),
        Box::new(lower_left),
    ];

    minefield
        .iter()
        .enumerate()
        .map(|(row_num, row)| {
            row.chars()
                .enumerate()
                .map(|(col_num, c)| {
                    if c == MINE {
                        MINE
                    } else {
                        let mine_count = locations
                            .iter()
                            .filter(|location| {
                                is_mine(minefield, location(row_num as isize, col_num as isize))
                            })
                            .count();
                        if mine_count == 0 {
                            ' '
                        } else {
                            char::from_digit(mine_count as u32, 10).unwrap()
                        }
                    }
                })
                .collect::<String>()
        })
        .collect()
}

fn is_mine(minefield: &[&str], (r, c): (isize, isize)) -> bool {
    r >= 0
        && c >= 0
        && (r as usize) < minefield.len()
        && (c as usize) < minefield[r as usize].len()
        && minefield[r as usize].as_bytes()[c as usize] == MINE as u8
}
