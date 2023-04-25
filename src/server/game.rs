use rand::random;
fn rand(max: usize) -> usize {random::<usize>() % max}

#[derive(Default)]
pub enum Cell {
    #[default]
    Empty,
    Miss,
    Hit,
    Ship,
}

impl std::fmt::Display for Cell{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Cell::*;
        write!(f, "{}", match self{
            Empty => "E",
            Miss  => "M",
            Hit   => "H",
            Ship  => "S",
        })
    }
}

#[derive(Default)]
pub struct Board(pub [[Cell;10];10]);

impl Board{
    pub const fn default() -> Self{
        use Cell::Empty as CE;
        Board([
            [CE, CE, CE, CE, CE, CE, CE, CE, CE, CE,],
            [CE, CE, CE, CE, CE, CE, CE, CE, CE, CE,],
            [CE, CE, CE, CE, CE, CE, CE, CE, CE, CE,],
            [CE, CE, CE, CE, CE, CE, CE, CE, CE, CE,],
            [CE, CE, CE, CE, CE, CE, CE, CE, CE, CE,],
            [CE, CE, CE, CE, CE, CE, CE, CE, CE, CE,],
            [CE, CE, CE, CE, CE, CE, CE, CE, CE, CE,],
            [CE, CE, CE, CE, CE, CE, CE, CE, CE, CE,],
            [CE, CE, CE, CE, CE, CE, CE, CE, CE, CE,],
            [CE, CE, CE, CE, CE, CE, CE, CE, CE, CE,],
        ])
    }

    pub fn init(&mut self) {
        let ships = vec![
            (rand(6), rand(6), rand(1)),
            (rand(6), rand(6), rand(1)),
            (rand(6), rand(6), rand(1)),
        ];

        for j in 0..3{
            let mut x  = ships[j].0;
            let mut y  = ships[j].1;
            for _ in 0..(j + 2){
                self.0[x][y] = Cell::Ship;
                if ships[0].2 == 1{
                    x += 1;
                }
                else{
                    y += 1;
                }
            }
        }
    }
}

impl std::fmt::Display for Board{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in &self.0{
            write!(f, "/")?;
            for r in c{
                write!(f, "{}", r)?;
            }
        }
        Ok(())
    }
}
