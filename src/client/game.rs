use rand::random as __rand;

fn rand() -> usize{
    __rand::<usize>() % 10
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum EnemyCell{
    #[default]
    Unknown,
    Hit,
    Miss,
}

impl std::fmt::Display for EnemyCell{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use EnemyCell::*;
        write!(f, "{}", match self {
            Hit => "x",
            Miss=> "o",
            _ => " "
        })
    }
}

#[derive(Default, Clone, Copy)]
pub enum PersonalCell{
    #[default]
    Empty,
    Hit,
    Miss,
    Ship,
}

impl std::fmt::Display for PersonalCell{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PersonalCell::*;
        write!(f, "{}", match self {
            Hit => "x",
            Ship=> "s",
            _ => " "
        })
    }
}

#[derive(Default, Clone, Copy)]
pub struct Board<CType>(pub [[CType; 10];10]);

pub type EnemyBoard = Board<EnemyCell>;
pub type PersonalBoard = Board<PersonalCell>;

impl EnemyBoard{
    pub const fn default() -> Self{
        use EnemyCell::Unknown as UnKn;
        Board([
            [UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn,],
            [UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn,],
            [UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn,],
            [UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn,],
            [UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn,],
            [UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn,],
            [UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn,],
            [UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn,],
            [UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn,],
            [UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn,],
        ])
    }


    pub fn decide_attack(&self) -> (usize, usize){
        let mut p = (0, 0);

        while self.0[p.0][p.1] != EnemyCell::Unknown{
            p.0 = rand();
            p.1 = rand();
        }


        p

    }
}

impl PersonalBoard{
    pub const fn default() -> Self{
        use PersonalCell::Empty as UnKn;
        Board([
            [UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn,],
            [UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn,],
            [UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn,],
            [UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn,],
            [UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn,],
            [UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn,],
            [UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn,],
            [UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn,],
            [UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn,],
            [UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn, UnKn,],
        ])
    }

    pub fn from_string(s: &String) -> Self{
        let mut board = Self::default();

        let mut c_index = 0;
        let mut r_index = 0;
        for letter in s.chars(){
            let v = match letter{
                'S' => PersonalCell::Ship,
                'E' => PersonalCell::Empty,
                'H' => PersonalCell::Hit,
                'M' => PersonalCell::Miss,
                '/' => {c_index = (c_index + 1) % 10; continue;},
                _ => panic!("Unknown letter")
            };
            board.0[c_index][r_index] = v;
            r_index = (r_index + 1) % 10;
        }

        board
    }
}


impl<T> std::fmt::Display for Board<T>
    where
        T: std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in &self.0{
            for r in c{
                write!(f, " {}", r)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
