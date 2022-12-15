use std::{collections::HashSet, cmp};
use scan_fmt::scan_fmt;

// Solve the problem!
pub fn input() -> &'static str { include_str!("../input/15") }
pub fn solve(input: &str) -> (usize, usize) {
    (p1(input, 2000000), p2(input, 4000000))
}

// Test the problem!
#[allow(dead_code)]
pub fn test_input() -> &'static str { include_str!("../input/15_test") }
#[allow(dead_code)]
pub fn test() {
    let input = test_input();
    assert_eq!(p1(input, 10), 26);
    assert_eq!(p2(input, 20), 56000011);
}

/* Solution Space Below. */

fn p1(input: &str, y: isize) -> usize {
    let mut intersections: HashSet<isize> = HashSet::new();
    for line in input.lines() {
        let (sx, sy, bx, by) = readline(line);
        let range = manhatten(sx, sy, bx, by);
        let dy = (y - sy).abs();
        if dy <= range {
            for x in (sx - range + dy)..(sx + range - dy) {
                intersections.insert(x);
            }
        }
    }
    intersections.len()
}

fn p2(input: &str, max: isize) -> usize {
    let mut options: HashSet<(isize, isize)> = HashSet::new();
    let mut sensors = vec![];
    let mut ranges = vec![];

    // Set up the list of possible locations of the hidden sensor
    // and also the sensor data we already know.
    for line in input.lines() {
        let (sx, sy, bx, by) = readline(line);
        let range = manhatten(sx, sy, bx, by);
        sensors.push((sx, sy));
        ranges.push(range);

        let dist = manhatten(sx, sy, bx, by) + 1;
        let x0 = cmp::max(0, cmp::min(sx - dist, max));
        let x1 = cmp::max(0, cmp::min(sx + dist, max));
        for x in x0..x1 {
            let t1 = (x, cmp::max(0, cmp::min(sy - (x - x0), max)));
            let t2 = (x, cmp::max(0, cmp::min(sy + (x - x0), max)));
            options.insert(t1);
            options.insert(t2);
        }  
    }

    // Check every possible location that we've narrowed down in the step above.
    let beacon = check_all(&sensors, &ranges, &options).unwrap();
    (beacon.0 * 4000000 + beacon.1) as usize
}

fn check_all(sensors: &Vec<(isize, isize)>, ranges: &Vec<isize>, options: &HashSet<(isize, isize)>) -> Option<(isize, isize)> {
    for (x, y) in options {
        let mut si = 0;
        let mut safe = true;
        while safe && si < sensors.len() {
            let dist = manhatten(*x, *y, sensors[si].0, sensors[si].1);
            if dist <= ranges[si] {
                safe = false;
            }
            si += 1;
        }
        if safe {
            return Some((*x, *y));
        }
    }
    None
}

fn manhatten(x0: isize, y0: isize, x1: isize, y1: isize) -> isize {
    (x0 - x1).abs() + (y0 - y1).abs()
}

fn readline(line: &str) -> (isize, isize, isize, isize) {
    scan_fmt!(
        line, "Sensor at x={}, y={}: closest beacon is at x={}, y={}", isize, isize, isize, isize).unwrap()
}