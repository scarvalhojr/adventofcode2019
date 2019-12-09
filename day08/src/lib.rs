use std::collections::HashMap;
use std::fmt::{Display, Formatter};

const BLACK: u8 = 0;
const WHITE: u8 = 1;
const TRANSPARENT: u8 = 2;

#[derive(Debug)]
pub struct Image {
    height: usize,
    width: usize,
    depth: usize,
    pixels: HashMap<(usize, usize, usize), u8>,
}

impl Image {
    pub fn new(height: usize, width: usize, values: &[u8]) -> Self {
        let mut depth = 0;
        let pixels = if width * height > 0 {
            depth = (values.len() as f32 / (width * height) as f32).ceil() as usize;
            values
                .chunks(height * width)
                .enumerate()
                .flat_map(|(layer, layer_values)| {
                    layer_values
                        .chunks(width)
                        .enumerate()
                        .flat_map(move|(row, row_values)| {
                            row_values
                                .iter()
                                .enumerate()
                                .map(move |(col, val)| ((layer, row, col), *val))
                        })
                })
                .collect()
            } else {
                HashMap::new()
            };

        Image {
            height,
            width,
            depth,
            pixels,
        }
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut result = vec![vec!['?'; self.width]; self.height];
        for layer in 0..self.depth {
            for row in 0..self.height {
                for col in 0..self.width {
                    if result[row][col] != '?' {
                        continue;
                    }
                    match self.pixels.get(&(layer, row, col)) {
                        Some(&BLACK) => result[row][col] = ' ',
                        Some(&WHITE) => result[row][col] = '#',
                        _ => {}
                    }
                }
            }
        }
        let image = result
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", image)
    }
}

pub fn part1(image: &Image) -> usize {
    let mut count = vec![0; image.depth];
    for layer in image
        .pixels
        .iter()
        .filter(|&(_, val)| *val == 0)
        .map(|((layer, _, _), _)| *layer) {
            count[layer] += 1;
        }
    match count.iter().enumerate().min_by_key(|(_, c)| *c) {
        Some((layer, _)) => {
            let ones = image
                .pixels
                .iter()
                .filter(|&((l, _, _), val)| *l == layer && *val == 1)
                .count();
            let twos = image
                .pixels
                .iter()
                .filter(|&((l, _, _), val)| *l == layer && *val == 2)
                .count();
            ones * twos
        },
        _ => 0,
    }
}

