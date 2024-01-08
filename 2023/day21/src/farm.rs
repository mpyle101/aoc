use std::convert::From;

pub const OPEN: u8 = 0b0000_0000;
pub const ROCK: u8 = 0b0000_0001;
pub const STEP: u8 = 0b0000_0010;
pub const MARK: u8 = 0b0000_0100;

#[derive(Clone, Debug)]
pub struct Garden {
    start: usize,
    ncols: usize,
    tiles: Vec<u8>,
    count: usize,
}
impl Garden {    
    pub fn len(&self) -> usize
    {
        self.tiles.len()
    }

    pub fn ncols(&self) -> usize
    {
        self.ncols
    }

    pub fn init(&mut self)
    {
        self.count = 1;
        self.tiles[self.start] = STEP
    }

    pub fn steps(&self) -> usize
    {
        self.count
    }

    #[inline]
    pub fn mark(&mut self, pos: usize)
    {
        if self.tiles[pos] & ROCK == 0 { self.tiles[pos] |= MARK }
    }

    #[inline]
    pub fn step(&mut self, pos: usize)
    {
        if self.tiles[pos] & ROCK == 0 { 
            self.tiles[pos] = STEP;
            self.count += 1;
        }
    }

    #[inline]
    pub fn clear(&mut self, pos: usize)
    {
        self.tiles[pos] &= !(1 << (STEP - 1))
    }

    #[inline]
    pub fn is_step(&self, pos: usize) -> bool
    {
        self.tiles[pos] & STEP == STEP
    }

    #[inline]
    pub fn update(&mut self)
    {
        self.count = 0;
        self.tiles.iter_mut()
            .filter(|c| **c & MARK == MARK)
            .for_each(|c| { *c = STEP; self.count += 1; });
    }

    pub fn print(&self)
    {
        self.tiles.iter()
            .enumerate()
            .for_each(|(i, c)| {
                if c & STEP == STEP {
                    print!("O")
                } else if c & ROCK == ROCK {
                    print!("#")
                } else if c & MARK == MARK {
                    print!("X")
                } else {
                    print!(".")
                }
                if (i + 1) % self.ncols == 0 {
                    println!()
                }
            }) 
    }
}
impl From<&str> for Garden {
    fn from(input: &str) -> Self
    {
        let mut ncols = 0;
        let mut start = 0;
    
        let tiles = input.lines()
            .enumerate()
            .flat_map(|(row, line)| {
                ncols = line.len();
                line.chars()
                    .enumerate()
                    .inspect(|(col, c)| if *c == 'S' { start = row * ncols + col })
                    .map(|(_, c)| match c {
                        '.' => OPEN,
                        '#' => ROCK,
                        'S' => OPEN,
                         _  => panic!("Unknown tile: {c}")
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
    
        Garden { start, ncols, tiles, count: 0 }
    }
} 
