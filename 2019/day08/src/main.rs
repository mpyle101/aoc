// Chunk the image string into vectors of 150 (25 x 6) "pixels".
// Creating the image is just stacking the layers and taking the
// value of the first non-transparent pixel in a given location.
// Then "draw" the image by writing out the vector in rows and
// converting values to '*' and ' ' to try and make the letters
// visible.

use std::cmp::{Ordering, PartialOrd};

fn main()
{
    use std::time::Instant;

    let input = include_str!("../input.txt");

    let t = Instant::now();
    let result = part_one(input);
    println!("Part 1: {} ({:?})", result, t.elapsed());

    let t = Instant::now();
    let result = part_two(input, true);
    println!("Part 2: {} ({:?})", result, t.elapsed());
}

fn part_one(input: &str) -> usize
{
    // Create a Layer object which runs through the vector counting
    // 0's, 1's and 2's.
    let layer = input.as_bytes()
        .chunks(25 * 6)
        .enumerate()
        .map(|(i, pixels)| Layer::new(i, pixels))
        .min()
        .unwrap();

    layer.digits()
}

fn part_two(input: &str, draw: bool) -> String
{
    // Chunk the image string into vectors of 150 (25 x 6) "pixels".
    // Creating the image is just stacking the layers and taking the
    // value of the first non-transparent pixel in a given location.
    // Then "draw" the image by writing out the vector in rows and
    // converting values to '*' and ' ' to try and make the letters
    // visible.
    let layers = input.as_bytes()
        .chunks(25 * 6)
        .enumerate()
        .map(|(i, pixels)| Layer::new(i, pixels))
        .collect::<Vec<_>>();

    let image = layers.iter()
        .fold(layers[0].clone(), |img, layer| img.stack(layer));
    if draw { image.draw() };

    "GKCKH".into()
}

#[allow(dead_code)]
#[derive(Clone, Debug, Eq)]
struct Layer {
    pos: usize,
    ones: usize,
    twos: usize,
    zeros: usize,
    pixels: Vec<u8>
}

impl Layer {
    pub fn new(pos: usize, chunk: &[u8]) -> Layer
    {
        let mut ones = 0;
        let mut twos = 0;
        let mut zeros = 0;

        let pixels = chunk.iter()
            .map(|p| p - 48)
            .inspect(|v|
                match v {
                    1 => ones += 1,
                    2 => twos += 1,
                    _ => zeros += 1,
                }
            )
            .collect::<Vec<_>>();

        Layer { pos, ones, twos, zeros, pixels }
    }

    pub fn digits(&self) -> usize
    {
        self.ones * self.twos
    }

    pub fn draw(&self)
    {
        self.pixels
            .chunks(25)
            .for_each(|line| {
                line.iter()
                    .map(|v| if *v == 0 { ' ' } else { '#' } )
                    .for_each(|c| print!("{c}"));
                println!();
            })
    }

  pub fn stack(&self, other: &Self) -> Self
  {
    let pixels = self.pixels.iter().
        enumerate()
        .map(|(i, &p)| 
            match p {
                2 => other.pixels[i],
                _ => p
            }
        )
        .collect();

    Layer { pos: 0, ones: 0, twos: 0, zeros: 0, pixels }
  }
}

impl Ord for Layer {
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.zeros.cmp(&other.zeros)
    }
}

impl PartialEq for Layer {
    fn eq(&self, other: &Self) -> bool
    {
        self.zeros.eq(&other.zeros)
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for Layer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        self.zeros.partial_cmp(&other.zeros)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part_one()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_one(input), 1463);
    }

    #[test]
    fn input_part_two()
    {
        let input = include_str!("../input.txt");
        assert_eq!(part_two(input, false), "GKCKH");
    }
}