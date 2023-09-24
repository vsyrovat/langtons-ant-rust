#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
    White,
    Black,
}
type Pos = (usize, usize); // x, y
type PngBuf = [u8; 131072]; // 1024 * 1024 / 8

pub struct Board {
    cells: Box<PngBuf>,
}

impl Board {
    pub fn new() -> Self {
        let cells = Box::new([255; 131072]);
        Board { cells }
    }

    fn offset(pos: &Pos) -> usize {
        ((pos.1 - 1) * 128) + ((pos.0 - 1) / 8)
    }

    fn bitshift(pos: &Pos) -> usize {
        7 - ((pos.0 - 1) % 8)
    }

    fn get_color(&self, pos: &Pos) -> Color {
        let offset = Self::offset(pos);
        let block = &self.cells[offset];
        let bitshift = Self::bitshift(pos);
        if (block & (1 << bitshift)) >> bitshift == 1 {
            Color::White
        } else {
            Color::Black
        }
    }

    pub fn set_color(&mut self, pos: &Pos, color: Color) {
        let offset = Self::offset(pos);
        let block = self.cells[offset];
        let bitshift = Self::bitshift(pos);
        match color {
            Color::White => {
                self.cells[offset] = block | (1 << bitshift);
            }
            Color::Black => {
                self.cells[offset] = block & !(1 << bitshift);
            }
        }
    }

    fn inbound(&self, pos: &Pos) -> bool {
        1 <= pos.0 && pos.0 <= 1024 && 1 <= pos.1 && pos.1 <= 1024
    }

    pub fn as_png_data(&self) -> PngBuf {
        *self.cells
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub struct Ant {
    direction: Direction,
    pos: Pos,
}

impl Ant {
    pub fn new() -> Self {
        Self {
            direction: Direction::Up,
            pos: (512, 512),
        }
    }
    fn rotate_cw(&mut self) {
        self.direction = match self.direction {
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
        }
    }
    fn rotate_ccw(&mut self) {
        self.direction = match self.direction {
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
        }
    }
}

pub struct Game {
    pub board: Board,
    pub ant: Ant,
    pub age: u64,
    pub black_count: u64,
}

impl Game {
    pub fn new(board: Board, ant: Ant) -> Self {
        Self {
            board,
            ant,
            age: 0,
            black_count: 0,
        }
    }

    pub fn step(&mut self) -> Option<()> {
        self.age = self.age + 1;
        let next_pos = self.next_cell_pos(&self.ant)?;
        let next_color = self.board.get_color(&next_pos);
        self.ant.pos = next_pos;
        match next_color {
            Color::White => {
                self.ant.rotate_cw();
                self.board.set_color(&next_pos, Color::Black);
                self.black_count += 1;
            }
            Color::Black => {
                self.ant.rotate_ccw();
                self.board.set_color(&next_pos, Color::White);
                self.black_count -= 1;
            }
        }
        Some(())
    }

    fn next_cell_pos(&self, ant: &Ant) -> Option<Pos> {
        let new_pos = match ant.direction {
            Direction::Up => (ant.pos.0, ant.pos.1 - 1),
            Direction::Down => (ant.pos.0, ant.pos.1 + 1),
            Direction::Right => (ant.pos.0 + 1, ant.pos.1),
            Direction::Left => (ant.pos.0 - 1, ant.pos.1),
        };
        self.board.inbound(&new_pos).then_some(new_pos)
    }

    pub fn play(&mut self, step_limit: u64) -> Option<()> {
        for _ in 0..step_limit {
            self.step()?;
        }
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Ant, Board, Color, Direction, Game, Pos};

    #[test]
    fn test_board() {
        let mut board = Board::new();
        assert_eq!(board.get_color(&(512, 512)), Color::White);
        board.set_color(&(512, 512), Color::Black);
        assert_eq!(board.get_color(&(512, 512)), Color::Black);
        board.set_color(&(512, 512), Color::White);
        assert_eq!(board.get_color(&(512, 512)), Color::White);
        assert_eq!(board.inbound(&(0, 0)), false);
        assert_eq!(board.inbound(&(1, 1)), true);
        assert_eq!(board.inbound(&(1024, 1024)), true);
        assert_eq!(board.inbound(&(1025, 1024)), false);
    }

    fn offset_and_bitshift(pos: &Pos) -> (usize, usize) {
        (Board::offset(pos), Board::bitshift(pos))
    }

    #[test]
    fn test_bitboard() {
        assert_eq!(offset_and_bitshift(&(1, 1)), (0, 7));
        assert_eq!(offset_and_bitshift(&(2, 1)), (0, 6));
        assert_eq!(offset_and_bitshift(&(1024, 1)), (127, 0));
        assert_eq!(offset_and_bitshift(&(15, 3)), (257, 1));
        assert_eq!(offset_and_bitshift(&(1, 1024)), (130944, 7));
        assert_eq!(offset_and_bitshift(&(1024, 1024)), (131071, 0));
    }

    #[test]
    fn test_step() {
        let board = Board::new();
        let ant = Ant::new();
        let mut game = Game::new(board, ant);

        game.step();
        assert_eq!(game.age, 1);
        assert_eq!(game.ant.direction, Direction::Right);
        assert_eq!(game.ant.pos, (512, 511));
        assert_eq!(game.board.get_color(&(512, 511)), Color::Black);

        game.step();
        assert_eq!(game.ant.direction, Direction::Down);
        assert_eq!(game.ant.pos, (513, 511));
        assert_eq!(game.board.get_color(&(513, 511)), Color::Black);

        game.step();
        game.step();
        game.step();
        assert_eq!(game.ant.direction, Direction::Left);
        assert_eq!(game.ant.pos, (512, 511));
        assert_eq!(game.board.get_color(&(512, 511)), Color::White);
    }

    #[test]
    fn test_play() {
        let board = Board::new();
        let ant = Ant::new();
        let mut game = Game::new(board, ant);

        game.play(5);
        assert_eq!(game.age, 5);
        assert_eq!(game.ant.direction, Direction::Left);
        assert_eq!(game.ant.pos, (512, 511));
        assert_eq!(game.board.get_color(&(512, 511)), Color::White);
        assert_eq!(game.black_count, 3);
    }
}
