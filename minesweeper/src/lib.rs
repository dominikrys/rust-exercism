use std::char;

#[rustfmt::skip]
static ADJACENT_OFFSETS: &'static [(i32, i32)] = &[
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len() as i32;
    (0..height)
        .map(|y| {
            let width = minefield[y as usize].len() as i32;
            (0..width)
                .map(|x| match minefield[y as usize].as_bytes()[x as usize] {
                    b'*' => '*',
                    _ => match ADJACENT_OFFSETS
                        .iter()
                        .map(|&(offset_x, offset_y)| (x + offset_x, y + offset_y))
                        .filter(|&(x, y)| (0 <= x && x < width) && (0 <= y && y < height))
                        .filter(|&(x, y)| minefield[y as usize].as_bytes()[x as usize] == b'*')
                        .count()
                    {
                        0 => ' ',
                        n => char::from_digit(n as u32, 10).unwrap(),
                    },
                })
                .collect()
        })
        .collect()
}
