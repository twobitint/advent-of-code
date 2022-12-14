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
    fn from(input: &str) -> Self {
        Self { data: Self::data(input) }
    }
    fn data(input: &str) -> PacketData {
        //println!("{}", input);
        if &input[0..1] != "[" {
            //println!("{}", input);
            return PacketData::Number((&input[0..input.len()]).parse().unwrap());
        }

        let mut iter = 1;
        let mut v = vec![];
        while iter < input.len() {
            //println!("{}", iter);
            let c = &input[iter..iter+1];
            //println!("c: {}", c);
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
                // println!("iter: {}", iter);
                // println!("chunk: {}", &input[iter..input.len()]);
                let end = (&input[iter..input.len()]).find(|s| { s == ',' || s == ']'}).unwrap() + iter;
                // println!("end: {}", end);
                // println!("l: {}", &input[iter..end]);
                v.push(Self::data(&input[iter..end]));
                iter = end;
            }
        }
        PacketData::List(v)

        // PacketData::List(
        //     input[1..input.len()-1]
        //         .split(",")
        //         .fold(vec![], |mut acc, i| {
        //             acc.push(Self::data(i));
        //             acc
        //         })
        // )
    }
}
impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        Ordering::Equal
        //(self.value, &self.name).cmp(&(other.value, &other.name))
    }
}
impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(Ordering::Equal)
        //self.data.partial_cmp(&other.data)
    }
}
impl PartialEq for PacketData {
    fn eq(&self, other: &Self) -> bool {
        true
        //self.data == other.data
    }
}
impl Eq for PacketData { }

fn p1(input: &str) -> usize {
    let mut sum = 0;
    let mut index = 1;
    for pair in input.split("\n\n") {
        let (p1, p2) = pair.split_once("\n").unwrap();
        let packet1 = Packet::from(p1);
        let packet2 = Packet::from(p2);
        //println!("{:#?}", packet1);
        sum += if packet1.data < packet2.data { index } else { 0 };
        index += 1;
    }
    sum
}