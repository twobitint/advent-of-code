use std::{ops::{Index, IndexMut}, collections::{HashSet, HashMap}, usize::MAX};

const MAX_GRID_ROW: usize = 100;
const START: char = 'S';
const START_ELEVATION: char = 'a';
const END: char = 'E';
const END_ELEVATION: char = 'z';

pub fn solve(input: &str) -> (usize, usize) {
    let map = Map::from(input);
    (map.path(true), map.path(false))
}

struct Map {
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
    data: [usize; MAX_GRID_ROW * MAX_GRID_ROW],
}
impl Default for Map {
    fn default() -> Self {
        Self { width: 0, height: 0, start: (0, 0), end: (0, 0), data: [0; MAX_GRID_ROW * MAX_GRID_ROW] }
    }
}
impl Index<(usize, usize)> for Map {
    type Output = usize;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.1 * MAX_GRID_ROW + index.0]
    }
}
impl IndexMut<(usize, usize)> for Map {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.1 * MAX_GRID_ROW + index.0]
    }
}
impl Map {
    fn from(input: &str) -> Self {
        let mut map = Self::default();

        let mut y = 0;
        for line in input.lines() {
            let mut x = 0;
            for char in line.chars() {
                let pos = (x, y);
                if char as char == START {
                    map.start = pos;
                    map[pos] = START_ELEVATION as usize;
                } else if char as char == END {
                    map.end = pos;
                    map[pos] = END_ELEVATION as usize;
                } else {
                    map[pos] = char as usize;
                }
                x += 1;
            }
            map.width = x;
            y += 1;
        }
        map.height = y;

        map
    }
    fn val(val: usize) -> Self {
        Self { data: [val; MAX_GRID_ROW * MAX_GRID_ROW], ..Default::default() }
    }
}

// Pathfinding.
impl Map {

    fn count_path<'a>(&self, came_from: &'a HashMap<(usize, usize), (usize, usize)>, mut current: &'a (usize, usize)) -> usize {
        let mut count = 0;
        while came_from.contains_key(current) {
            current = &came_from[current];
            count += 1;
        }
        count
    }

    fn path(&self, use_starting_position: bool) -> usize {

        let mut open_set = HashSet::new();
        let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut g_score = Map::val(MAX);
        let mut f_score = Map::val(MAX);

        if use_starting_position {
            open_set.insert(self.start);
            g_score[self.start] = 0;
            f_score[self.start] = self.h(self.start);
        } else {
            for y in 0..self.height {
                for x in 0..self.width {
                    let pos = (x, y);
                    if self[pos] == START_ELEVATION as usize {
                        open_set.insert(pos);
                        g_score[pos] = 0;
                        f_score[pos] = self.h(pos);
                    }
                }
            }
        }
        
        while open_set.len() != 0 {
            let current = self.current(&open_set, &f_score);
            if current == self.end {
                return self.count_path(&came_from, &current);
            }

            open_set.remove(&current);
            for neighbor in self.neighbors(&current) {
                let tentative_g_score = g_score[current] + self.d(&current, &neighbor);
                if tentative_g_score < g_score[neighbor] {
                    came_from.insert(neighbor, current);
                    g_score[neighbor] = tentative_g_score;
                    f_score[neighbor] = tentative_g_score + self.h(neighbor);
                    if !open_set.contains(&neighbor) {
                        open_set.insert(neighbor);
                    }
                }
            }
        }
        return 0;
    }

    fn neighbors(&self, pos: &(usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = *pos;
        let mut neighbors = vec![];

        // Max elevation allowed
        let el = self[*pos] + 1;

        if x > 0 && self[(x - 1, y)] <= el {
            neighbors.push((x - 1, y));
        }
        if x < self.width - 1 && self[(x + 1, y)] <= el {
            neighbors.push((x + 1, y));
        }
        if y > 0 && self[(x, y - 1)] <= el {
            neighbors.push((x, y - 1));
        }
        if y < self.height - 1 && self[(x, y + 1)] <= el {
            neighbors.push((x, y + 1));
        }

        neighbors
    }

    // Get node in open set with lowest f_score.
    // Could use a minheap here instead.
    fn current(&self, open_set: &HashSet<(usize, usize)>, f_score: &Self) -> (usize, usize) {
        let mut min_f = MAX;
        let mut min_pos = (0, 0);
        for pos in open_set {
            if f_score[*pos] < min_f {
                min_f = f_score[*pos];
                min_pos = *pos;
            }
        }
        min_pos
    }

    fn h(&self, pos: (usize, usize)) -> usize {
        let dx = if self.end.0 > pos.0 { self.end.0 - pos.0 } else { pos.0 - self.end.0 };
        let dy = if self.end.1 > pos.1 { self.end.1 - pos.1 } else { pos.1 - self.end.1 };
        dx + dy
    }

    fn d(&self, _: &(usize, usize), _: &(usize, usize)) -> usize { 1 }

}