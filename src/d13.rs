use std::cmp::Ordering;

// Solve the problem!
pub fn input() -> &'static str { include_str!("../input/13") }
pub fn solve(input: &str) -> (usize, usize) {
    (p1(input), p2(input))
}

// Test the problem!
#[allow(dead_code)]
pub fn test_input() -> &'static str { include_str!("../input/13_test") }
#[allow(dead_code)]
pub fn test() {
    let input = test_input();
    let (p1, p2) = solve(input);
    assert_eq!(p1, 13);
    assert_eq!(p2, 140);
}

/* Solution Space Below. */

#[derive(Debug)]
enum PacketData {
    Number(usize),
    List(Vec<PacketData>),
}
#[derive(Debug)]
struct Packet {
    data: PacketData,
    divider: bool,
}
impl Packet {
    fn in_order(p1: &Vec<PacketData>, p2: &Vec<PacketData>) -> Option<bool> {
        let mut iter = 0;

        while iter < p1.len() && iter < p2.len() {

            let in_order = match (&p1[iter], &p2[iter]) {
                (PacketData::Number(a), PacketData::Number(b)) => {
                    if a < b {
                        Some(true)
                    } else if a == b {
                        None
                    } else {
                        Some(false)
                    }
                },
                (PacketData::Number(a), PacketData::List(b)) => {
                    //let np1 = PacketData::List(vec![PacketData::Number(*a)]);
                    Self::in_order(&vec![PacketData::Number(*a)], b)
                },
                (PacketData::List(a), PacketData::Number(b)) => {
                    Self::in_order(a, &vec![PacketData::Number(*b)])
                },
                (PacketData::List(a), PacketData::List(b)) => {
                    Self::in_order(a, b)
                }
            };

            if in_order != None {
                return in_order;
            } else {
                iter += 1;
            }
        }

        if iter < p1.len() {
            Some(false)
        } else if iter < p2.len() {
            Some(true)
        } else {
            None
        }

    }

    fn from(input: &str) -> Self {
        Self { data: Self::data(input), divider: false }
    }
    fn data(input: &str) -> PacketData {
        if &input[0..1] != "[" {
            return PacketData::Number((&input[0..input.len()]).parse().unwrap());
        }
        let mut iter = 1;
        let mut v = vec![];
        while iter < input.len() {
            let c = &input[iter..iter+1];
            if c == "[" {
                let mut open_count = 1;
                let start = iter;
                while open_count > 0 {
                    iter += 1;
                    if &input[iter..iter+1] == "[" {
                        open_count += 1;
                    } else if &input[iter..iter+1] == "]" {
                        open_count -= 1;
                    }
                }
                iter += 1;
                v.push(Self::data(&input[start..iter]));
            } else if c == "," || c == "]" {
                iter += 1;
            } else {
                let end = (&input[iter..input.len()]).find(|s| { s == ',' || s == ']'}).unwrap() + iter;
                v.push(Self::data(&input[iter..end]));
                iter = end;
            }
        }
        PacketData::List(v)
    }
}

fn p1(input: &str) -> usize {
    let mut sum = 0;
    let mut index = 1;
    for pair in input.split("\n\n") {
        let (p1, p2) = pair.split_once("\n").unwrap();
        let packet1 = Packet::from(p1);
        let packet2 = Packet::from(p2);

        let (pv1, pv2) = match (packet1.data, packet2.data) {
            (PacketData::List(a), PacketData::List(b)) => { (a, b) },
            _ => { (vec![], vec![]) }
        };

        sum += if Packet::in_order(&pv1, &pv2).unwrap() { index } else { 0 };
        index += 1;
    }
    sum
}

fn p2(input: &str) -> usize {
    let mut packets = vec![];
    for pair in input.split("\n\n") {
        let (p1, p2) = pair.split_once("\n").unwrap();
        let packet1 = Packet::from(p1);
        let packet2 = Packet::from(p2);
        packets.push(packet1);
        packets.push(packet2);
    }

    // Insert dividers.
    packets.push(Packet {divider: true, data: PacketData::List(vec![PacketData::List(vec![PacketData::Number(2)])]) });
    packets.push(Packet {divider: true, data: PacketData::List(vec![PacketData::List(vec![PacketData::Number(6)])]) });

    // Sort the packets.
    packets.sort_by(|a, b| {
        let mut pv1 = &vec![];
        let mut pv2 = &vec![];
        match (&a.data, &b.data) {
            (PacketData::List(a), PacketData::List(b)) => {
                pv1 = a;
                pv2 = b;
            },
            _ => {}
        };
        match Packet::in_order(pv1, pv2) {
            None => { Ordering::Equal },
            Some(v) => { if v { Ordering::Less } else { Ordering::Greater }}
        }
    });

    // Find dividers.
    let mut d1 = 0;
    let mut d2 = 0;
    let mut iter = 0;
    while d1 == 0 || d2 == 0 {
        if let Some(packet) = packets.get(iter) {
            if packet.divider {
                if d1 == 0 {
                    d1 = iter + 1;
                } else {
                    d2 = iter + 1;
                }
            }
        }
        iter += 1;
    }
    d1 * d2
}