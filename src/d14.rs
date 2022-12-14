use std::{ops::{Index, IndexMut}, usize::MAX};

use scan_fmt::scan_fmt;

// Solve the problem!
pub fn input() -> &'static str { include_str!("../input/14") }
pub fn solve(input: &str) -> (usize, usize) {
    (p1(input), p2(input))
}

// Test the problem!
#[allow(dead_code)]
pub fn test_input() -> &'static str { include_str!("../input/14_test") }
#[allow(dead_code)]
pub fn test() {
    let input = test_input();
    let (p1, p2) = solve(input);
    assert_eq!(p1, 24);
    assert_eq!(p2, 93);
}

/* Solution Space Below. */

const SAND_SOURCE: (usize, usize) = (500, 0);
const MAX_CAVE_WIDTH: usize = 1000;
const MAX_CAVE_HEIGHT: usize = 500;

fn p1(input: &str) -> usize {
    let mut cave = Cave::from(input);
    cave.simulate();
    cave.sand_count
}

fn p2(input: &str) -> usize {
    let mut cave = Cave::from(input);
    cave.add_floor();
    cave.simulate();
    cave.sand_count + 1
}

#[derive(Clone, Copy)]
enum Material {
    Air,
    Rock,
    Sand,
}
struct Cave {
    width: usize,
    height: usize,
    source: (usize, usize),
    sand_count: usize,
    data: [Material; MAX_CAVE_WIDTH * MAX_CAVE_HEIGHT],
    min: (usize, usize),
    max: (usize, usize),
}
impl Default for Cave {
    fn default() -> Self {
        Self { width: 0, height: 0, source: SAND_SOURCE, sand_count: 0, data: [Material::Air; MAX_CAVE_WIDTH * MAX_CAVE_HEIGHT], min: (MAX, MAX), max: (0,0) }
    }
}
impl Index<(usize, usize)> for Cave {
    type Output = Material;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.1 * MAX_CAVE_WIDTH + index.0]
    }
}
impl IndexMut<(usize, usize)> for Cave {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.1 * MAX_CAVE_WIDTH + index.0]
    }
}
impl Cave {
    fn print(&self) {
        println!("{:?}, {:?}", self.min, self.max);
        for y in self.min.1..self.max.1+1 {
            for x in self.min.0..self.max.0+1 {
                match self[(x, y)] {
                    Material::Air => { print!(".") },
                    Material::Rock => { print!("#") },
                    Material::Sand => { print!("o") },
                }
            }
            println!();
        }
    }

    fn simulate(&mut self) { while self.simulate_once() {} }
    fn simulate_once(&mut self) -> bool {
        let mut pos = SAND_SOURCE;
        loop {
            let next = self.move_sand(pos);
            if next.1 == MAX_CAVE_HEIGHT - 1 || next == SAND_SOURCE {
                return false;
            }
            if next == pos {
                self[pos] = Material::Sand; 
                self.sand_count += 1;

                let (x, y) = pos;
                if x > self.max.0 {
                    self.max.0 = x;
                }
                if x < self.min.0 {
                    self.min.0 = x;
                }
                if y > self.max.1 {
                    self.max.1 = y;
                }
                if y < self.min.1 {
                    self.min.1 = y;
                }

                //self.print();
                return true;
            }
            pos = next;
        }
    }
    fn move_sand(&self, pos: (usize, usize)) -> (usize, usize) {
        let mut mov = pos;
        mov.1 += 1;
        if mov.1 == MAX_CAVE_HEIGHT { return pos; }
        if matches!(self[mov], Material::Air) {
            return mov;
        } else {
            mov.0 -= 1;
            if matches!(self[mov], Material::Air) {
                return mov;
            } else {
                mov.0 += 2;
                return if matches!(self[mov], Material::Air) { mov } else { pos };
            }
        }
    }

    fn add_floor(&mut self) {
        let y = self.max.1 + 2;
        for x in 0..MAX_CAVE_WIDTH {
            self[(x, y)] = Material::Rock;
        }
    }

    fn from(input: &str) -> Self {
        let mut cave = Self::default();
        for line in input.lines() {
            let mut last_x: Option<usize> = None;
            let mut last_y: Option<usize> = None;
            for coord in line.split(" -> ") {
                let (x, y) = scan_fmt!(coord, "{},{}", usize, usize).unwrap();

                if x > cave.max.0 {
                    cave.max.0 = x;
                }
                if x < cave.min.0 {
                    cave.min.0 = x;
                }
                if y > cave.max.1 {
                    cave.max.1 = y;
                }
                if y < cave.min.1 {
                    cave.min.1 = y;
                }

                if let (Some(last_x), Some(last_y)) = (last_x, last_y) {
                    let x0 = if x > last_x { last_x } else { x };
                    let x1 = if x > last_x { x } else { last_x };
                    let y0 = if y > last_y { last_y } else { y };
                    let y1 = if y > last_y { y } else { last_y };
                    for xi in x0..x1+1 {
                        cave[(xi, y)] = Material::Rock;
                    }
                    for yi in y0..y1+1 {
                        cave[(x, yi)] = Material::Rock;
                    }
                }
                last_x = Some(x);
                last_y = Some(y);
            }
        }
        cave
    }
}