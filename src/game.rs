use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct World {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug)]
pub struct Snake {
    pub body: VecDeque<Coord>,
    pub dir: Dir,
    pending_dir: Dir,
    grow: bool,
}

#[derive(Debug)]
pub struct Game {
    pub world: World,
    pub snake: Snake,
    pub food: Coord,
    rng: StdRng,
    pub score: u64,
    pub game_over: bool,
}

impl World {
    pub fn contains(&self, c: Coord) -> bool {
        c.x >= 0 && c.x < self.width && c.y >= 0 && c.y < self.height
    }
}

impl Snake {
    pub fn new(start: Coord, len: usize, dir: Dir) -> Self {
        let mut body = VecDeque::with_capacity(len + 8);
        for i in 0..len {
            body.push_back(Coord { x: start.x - i as i32, y: start.y });
        }
        Self { body, dir, pending_dir: dir, grow: false }
    }

    pub fn head(&self) -> Coord { *self.body.front().unwrap() }

    pub fn set_dir(&mut self, dir: Dir) {
        let illegal = matches!(
            (self.dir, dir),
            (Dir::Up, Dir::Down) | (Dir::Down, Dir::Up) | (Dir::Left, Dir::Right) | (Dir::Right, Dir::Left)
        );
        if !illegal {
            self.pending_dir = dir;
        }
    }

    pub fn step(&mut self) {
        self.dir = self.pending_dir;
        let mut next = self.head();
        match self.dir {
            Dir::Up => next.y -= 1,
            Dir::Down => next.y += 1,
            Dir::Left => next.x -= 1,
            Dir::Right => next.x += 1,
        }
        self.body.push_front(next);
        if !self.grow {
            self.body.pop_back();
        } else {
            self.grow = false;
        }
    }

    pub fn grow(&mut self) { self.grow = true; }

    pub fn hits_self(&self) -> bool {
        let h = self.head();
        self.body.iter().skip(1).any(|&c| c == h)
    }
}

impl Game {
    pub fn new(world_w: i32, world_h: i32, seed: u64) -> Self {
        let world = World { width: world_w, height: world_h };
        let rng = StdRng::seed_from_u64(seed);
        let start = Coord { x: world_w / 2, y: world_h / 2 };
        let mut g = Self {
            food: Coord { x: 0, y: 0 },
            world,
            snake: Snake::new(start, 4, Dir::Right),
            rng,
            score: 0,
            game_over: false,
        };
        g.food = g.rand_free_cell();
        g
    }

    pub fn update(&mut self) {
        if self.game_over { return; }
        self.snake.step();

        let head = self.snake.head();
        if !self.world.contains(head) || self.snake.hits_self() {
            self.game_over = true;
            return;
        }
        if head == self.food {
            self.snake.grow();
            self.score += 1;
            self.food = self.rand_free_cell();
        }
    }

    pub fn change_dir(&mut self, dir: Dir) { self.snake.set_dir(dir); }

    fn rand_free_cell(&mut self) -> Coord {
        loop {
            let x = self.rng.gen_range(0..self.world.width);
            let y = self.rng.gen_range(0..self.world.height);
            let c = Coord { x, y };
            if !self.snake.body.iter().any(|&b| b == c) {
                return c;
            }
        }
    }
}
