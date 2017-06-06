fn index(width: usize, x: usize, y: usize) -> usize {
    y * width + x
}

pub struct World {
    pub width: usize,
    pub height: usize,
    cells: Vec<i32>,
}

impl World {
    pub fn new(width: usize, height: usize) -> World {
        let size = (width * height) as usize;
        let mut cells = vec![0; size];

        for y in 0..height {
            for x in 0..width {
                if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                    cells[index(width, x, y)] = 1;
                }
            }
        }

        World {
            width: width,
            height: height,
            cells: cells,
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> i32 {
        let index = (y * self.width + x) as usize;
        self.cells[index]
    }
}

pub struct SnakeSegment {
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
}

impl SnakeSegment {
    pub fn new(x: usize, y: usize, direction: Direction) -> SnakeSegment {
        SnakeSegment {
            x: x,
            y: y,
            direction: direction,
        }
    }

    pub fn move_fwd(&mut self) -> () {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
        }
    }

    pub fn turn(&mut self, direction: Direction) -> () {
        self.direction = direction;
    }
}

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
