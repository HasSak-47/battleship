#[derive(Default)]
pub enum Cell {
    #[default]
    Empty,
    Hit,
    Ship,
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
}
