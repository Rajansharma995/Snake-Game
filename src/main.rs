extern crate piston_window;
extern crate rand;

use piston_window::*;
use piston_window::types::Color;
use rand::{thread_rng, Rng};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    body: Vec<(i32, i32)>,
    direction: Direction,
}

struct Game {
    snake: Snake,
    food_x: i32,
    food_y: i32,
    width: i32,
    height: i32,
    game_over: bool,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl Snake {
    fn move_forward(&mut self, dir: Option<Direction>) {
        let head = self.body[0];
        let new_head = match dir {
            Some(Direction::Up) => (head.0, head.1 - 1),
            Some(Direction::Down) => (head.0, head.1 + 1),
            Some(Direction::Left) => (head.0 - 1, head.1),
            Some(Direction::Right) => (head.0 + 1, head.1),
            None => head,
        };
        self.body.insert(0, new_head);
    }

    fn head_direction(&self) -> Direction {
        self.direction.clone()
    }

    fn overlap_tail(&self, x: i32, y: i32) -> bool {
        self.body.iter().any(|&pos| pos == (x, y))
    }
}

impl Game {
    fn handle_key_event(&mut self, key: Key) {
        if !self.game_over {
            let dir = match key {
                Key::Up => Some(Direction::Up),
                Key::Down => Some(Direction::Down),
                Key::Left => Some(Direction::Left),
                Key::Right => Some(Direction::Right),
                _ => Some(self.snake.head_direction()),
            };

            if let Some(dir) = dir {
                if dir == self.snake.head_direction().opposite() {
                    return;
                }
            }

            self.snake.move_forward(dir.clone());

        }
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if !self.game_over {
            if self.check_if_snake_alive(dir.clone()) {
                self.snake.move_forward(dir.clone());
                self.check_eating();
            } else {
                self.game_over = true;
            }
        }
    }
    

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let head = self.snake.body[0];
        let (x, y) = match dir {
            Some(Direction::Up) => (head.0, head.1 - 1),
            Some(Direction::Down) => (head.0, head.1 + 1),
            Some(Direction::Left) => (head.0 - 1, head.1),
            Some(Direction::Right) => (head.0 + 1, head.1),
            None => (head.0, head.1), 

        if x < 0 || x >= self.width || y < 0 || y >= self.height || self.snake.overlap_tail(x, y) {
            false
        } else {
            true
        }
    }

    fn check_eating(&mut self) {
        let head = self.snake.body[0];
        if head.0 == self.food_x && head.1 == self.food_y {
            self.snake.body.push((self.food_x, self.food_y));
            self.add_food();
        }
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1..self.width - 1);
        let mut new_y = rng.gen_range(1..self.height - 1);
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.height - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
    }

    fn restart(&mut self) {
        self.snake = Snake {
            body: vec![(2, 2)],
            direction: Direction::Right,
        };
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
    }
}

fn main() {
    let width = 30;
    let height = 20;

    let mut window: PistonWindow =
        WindowSettings::new("Rust Snake Game", [width as u32 * 20, height as u32 * 20])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        snake: Snake {
            body: vec![(2, 2)],
            direction: Direction::Right,
        },
        food_x: 6,
        food_y: 4,
        width,
        height,
        game_over: false,
    };

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.handle_key_event(key);
        }

        window.draw_2d(&event, |c, g, _| {
            clear(BORDER_COLOR, g);

            if !game.game_over {
                for block in &game.snake.body {
                    rectangle(
                        FOOD_COLOR,
                        [block.0 as f64 * 20.0, block.1 as f64 * 20.0, 20.0, 20.0],
                        c.transform,
                        g,
                    );
                }

                rectangle(
                    FOOD_COLOR,
                    [
                        game.food_x as f64 * 20.0,
                        game.food_y as f64 * 20.0,
                        20.0,
                        20.0,
                    ],
                    c.transform,
                    g,
                );
            }
        });
    }
}
