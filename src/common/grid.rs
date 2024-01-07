use std::{
    fmt::{Debug, Display},
    ops::Rem,
};

use itertools::Itertools;

#[derive(Debug)]
pub struct SquareGrid<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

impl<T> SquareGrid<T> {
    pub fn contains(&self, x: usize, y: usize) -> bool {
        (0..self.width).contains(&x) && (0..self.height).contains(&y)
    }

    #[allow(dead_code)]
    pub fn get_pos(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if self.contains(x, y) {
            self.data.get(y * self.width + x)
        } else {
            None
        }
    }

    pub fn get_infinitely_looping(&self, x: i64, y: i64) -> Option<&T> {
        let x = x.rem(self.width as i64) as usize;
        let y = y.rem(self.height as i64) as usize;
        self.get(x, y)
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) -> bool {
        if self.contains(x, y) {
            self.data[y * self.width + x] = value;
            true
        } else {
            false
        }
    }

    pub fn travel(&self, x: usize, y: usize, direction: Direction) -> Option<(usize, usize)> {
        if self.contains(x, y) {
            match direction {
                Direction::Up => {
                    if y == 0 {
                        None
                    } else {
                        Some((x, y - 1))
                    }
                }
                Direction::Down => Some((x, y + 1)),
                Direction::Left => {
                    if x == 0 {
                        None
                    } else {
                        Some((x - 1, y))
                    }
                }
                Direction::Right => Some((x + 1, y)),
            }
            .filter(|(x2, y2)| self.contains(*x2, *y2))
        } else {
            None
        }
    }

    pub fn neighbors_8(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        (0..3)
            .cartesian_product(0..3)
            .filter(|(dx, dy)| {
                (x + dx) != 0
                    && (y + dy) != 0
                    && (x + dx) <= self.width
                    && (y + dy) <= self.height
                    && (dx != &1 || dy != &1)
            })
            .map(|(dx, dy)| (x + dx - 1, y + dy - 1))
            .collect_vec()
    }

    pub fn neighbors_4(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        self.neighbors_8(x, y)
            .into_iter()
            .filter(|(x2, y2)| x == *x2 || y == *y2)
            .collect_vec()
    }
}

impl<T> Display for SquareGrid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.get(x, y).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Direction::Up => "^",
            Direction::Down => "v",
            Direction::Left => "<",
            Direction::Right => ">",
        })
    }
}
