use std::{ops::{Index, IndexMut}, collections::{HashSet, HashMap, BinaryHeap}, usize::MAX};

const MAX_GRID_ROW: usize = 100;
const START: char = 'S';
const START_ELEVATION: char = 'a';
const END: char = 'E';
const END_ELEVATION: char = 'z';

// Solve the problem!
pub fn input() -> &'static str { include_str!("../input/12") }
pub fn solve(input: &str) -> (usize, usize) {
    let map = Map::from(input);
    (map.path(true), map.path(false))
}

// Test the problem!
#[allow(dead_code)]
pub fn test_input() -> &'static str { include_str!("../input/12_test") }
#[allow(dead_code)]
pub fn test() {
    let input = test_input();
    let (p1, p2) = solve(input);
    assert_eq!(p1, 31);
    assert_eq!(p2, 29);
}

// Our custom indexable map which will store location
// and elevation data for finding a shortest path.
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
    // Build a new map from the given problem input, which is a
    // string represtation of elevations using characters for
    // height, and starting and ending locations.
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
    // We use this struct in our pathfinding algorithm as well, so allow
    // it to be initialized with any usize we want.
    fn val(val: usize) -> Self {
        Self { data: [val; MAX_GRID_ROW * MAX_GRID_ROW], ..Default::default() }
    }
}

// Pathfinding.
impl Map {

    // Solve the problem. Count the total number of steps for the
    // minimum travel path from start to finish. For part 2, a boolean
    // is passed to decide whether starting position matters.
    //
    // Here we use an A* pathfinding algorithm, though since all nodes
    // are equidistant, this is likely overkill. Could use djykstra or
    // a simple BFS.
    fn path(&self, use_starting_position: bool) -> usize {

        let mut open_set = BinaryHeap::new();
        let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut g_score = Map::val(MAX);
        let mut f_score = Map::val(MAX);

        if use_starting_position {
            open_set.push(self.start); // insert(self.start);
            g_score[self.start] = 0;
            f_score[self.start] = self.h(self.start);
        } else {
            for y in 0..self.height {
                for x in 0..self.width {
                    let pos = (x, y);
                    if self[pos] == START_ELEVATION as usize {
                        open_set.push(pos); // insert(pos);
                        g_score[pos] = 0;
                        f_score[pos] = self.h(pos);
                    }
                }
            }
        }
        
        while open_set.len() != 0 {
            //let current = self.current(&open_set, &f_score);
            let current = open_set.pop().unwrap();
            if current == self.end {
                return self.count_path(&came_from, &current);
            }

            // open_set.remove(&current);
            for neighbor in self.neighbors(&current) {
                let tentative_g_score = g_score[current] + self.d(&current, &neighbor);
                if tentative_g_score < g_score[neighbor] {
                    came_from.insert(neighbor, current);
                    g_score[neighbor] = tentative_g_score;
                    f_score[neighbor] = tentative_g_score + self.h(neighbor);
                    if !self.open_set_contains(&open_set, &neighbor) {
                    //if !open_set.contains(&neighbor) {
                        open_set.push(neighbor); //open_set.insert(neighbor);
                    //}
                    }
                }
            }
        }
        return 0;
    }

    fn open_set_contains(&self, open_set: &BinaryHeap<(usize, usize)>, pos: &(usize, usize)) -> bool {
        for other in open_set {
            if other == pos {
                return true;
            }
        }
        false
    }

    // Get node in open set with lowest f_score.
    // Could use a minheap here instead of traversing the whole list every time.
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

    // Count up the total number of nodes from start to finish that has the shortest path.
    // For the sake of this problem, we don't need to know the actual path, but just the
    // total number of moves.
    fn count_path<'a>(&self, came_from: &'a HashMap<(usize, usize), (usize, usize)>, mut current: &'a (usize, usize)) -> usize {
        let mut count = 0;
        while came_from.contains_key(current) {
            current = &came_from[current];
            count += 1;
        }
        count
    }

    // Get the possible neighbors from a given position. This can be
    // one of the four cardinal directions as long as the neighbor's
    // elevation isn't more than a single step higher.
    fn neighbors(&self, pos: &(usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = *pos;
        let mut neighbors = vec![];

        // Max elevation allowed
        let max = self[*pos] + 1;

        if x > 0 && self[(x - 1, y)] <= max {
            neighbors.push((x - 1, y));
        }
        if x < self.width - 1 && self[(x + 1, y)] <= max {
            neighbors.push((x + 1, y));
        }
        if y > 0 && self[(x, y - 1)] <= max {
            neighbors.push((x, y - 1));
        }
        if y < self.height - 1 && self[(x, y + 1)] <= max {
            neighbors.push((x, y + 1));
        }

        neighbors
    }

    // The heuristic to calculate travel distance remaining.
    // This is a simple manhatten distance function.
    fn h(&self, pos: (usize, usize)) -> usize {
        let dx = if self.end.0 > pos.0 { self.end.0 - pos.0 } else { pos.0 - self.end.0 };
        let dy = if self.end.1 > pos.1 { self.end.1 - pos.1 } else { pos.1 - self.end.1 };
        dx + dy
    }

    // The cost to move from one position to another.
    // All adjacent positions are simply one unit apart.
    fn d(&self, _: &(usize, usize), _: &(usize, usize)) -> usize { 1 }

}