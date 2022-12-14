use std::cmp::Ordering;

// Solve the problem!
pub fn input() -> &'static str { include_str!("../input/13") }
pub fn solve(input: &str) -> (usize, usize) {
    (p1(input), 0)
}

// Test the problem!
#[allow(dead_code)]
pub fn test_input() -> &'static str { include_str!("../input/13_test") }
#[allow(dead_code)]
pub fn test() {
    let input = test_input();
    let (p1, p2) = solve(input);
    assert_eq!(p1, 13);
    assert_eq!(p2, 0);
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
}
impl Packet {
    fn in_order(p1: &PacketData, p2: &PacketData) -> bool {
        match p1 {
            PacketData::Number(a) => match p2 {
                PacketData::Number(b) => { a < b },
                PacketData::List(b) => {
                    //let np1 = PacketData::List(vec![PacketData::Number(*a)]);
                    Self::in_order(&PacketData::List(vec![PacketData::Number(*a)]), p2)
                }
            }
            PacketData::List(a) => match p2 {
                PacketData::Number(b) => {
                    Self::in_order(p1, &PacketData::List(vec![PacketData::Number(*b)]))
                },
                PacketData::List(b) => {
                    if b.len() < a.len() {
                        return false;
                    }
                    let mut result = true;
                    let mut iter = 0;
                    for item in a {
                        result = result && Self::in_order(item, b.get(iter).unwrap());
                        iter += 1;
                    }
                    return result;
                },
            }
        }
    }

    fn from(input: &str) -> Self {
        Self { data: Self::data(input) }
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
// impl Ord for PacketData {
//     fn cmp(&self, other: &Self) -> Ordering {
//         Ordering::Equal
//         //(self.value, &self.name).cmp(&(other.value, &other.name))

//     }
// }
// impl PartialOrd for PacketData {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(Ordering::Equal)
//         //self.data.partial_cmp(&other.data)
//     }
// }
// impl PartialEq for PacketData {
//     fn eq(&self, other: &Self) -> bool {
//         true
//         //self.data == other.data
//     }
// }
// impl Eq for PacketData { }

fn p1(input: &str) -> usize {
    let mut sum = 0;
    let mut index = 1;
    for pair in input.split("\n\n") {
        let (p1, p2) = pair.split_once("\n").unwrap();
        let packet1 = Packet::from(p1);
        let packet2 = Packet::from(p2);

        let o = Packet::in_order(&packet1.data, &packet2.data);
        println!("{}", pair);
        println!("{}", o);

        sum += if Packet::in_order(&packet1.data, &packet2.data) { index } else { 0 };
        index += 1;
    }
    sum
}