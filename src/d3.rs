pub fn solve(input: &str) -> (u16, u16) {

    let mut g0: &[u8] = &[0];
    let mut g1: &[u8] = &[0];

    let mut iter = 1;
    return input.lines().fold((0, 0), |acc, line| {

        let ruck = line.as_bytes(); // -65 to get to 0
        
        let p1 = part1(ruck);

        let mut p2 = 0;
        if iter % 3 == 0 {
            p2 = part2(g0, g1, ruck);
        } else if iter % 2 == 0 {
            g1 = ruck;
        } else {
            g0 = ruck;
        }
        iter += 1;

        return (acc.0 + p1, acc.1 + p2);
    });

}

fn part2(g0: &[u8], g1: &[u8], g2: &[u8]) -> u16 {
    
    let mut index = 0;
    let mut items: [(u8, u8, u8); 53] = [(0, 0, 0); 53];
    let len = *Vec::from([g0.len(), g1.len(), g2.len()]).iter().max().unwrap();

    for iter in 0..len {
        if iter < g0.len() {
            index = atop(g0[iter]) as usize;
            items[index].0 = 1;
            if items[index].1 == 1 && items[index].2 == 1 {
                return index as u16;
            }
        }
        if iter < g1.len() {
            index = atop(g1[iter]) as usize;
            items[index].1 = 1;
            if items[index].0 == 1 && items[index].2 == 1 {
                return index as u16;
            }
        }
        if iter < g2.len() {
            index = atop(g2[iter]) as usize;
            items[index].2 = 1;
            if items[index].0 == 1 && items[index].1 == 1 {
                return index as u16;
            }
        }
    }

    return index as u16;
}

fn part1(ruck: &[u8]) -> u16 {
    let mut items: [(u8, u8); 53] = [(0, 0); 53];
    let mut index: usize = 0;

    for iter in 0..ruck.len() {

        if iter % 2 == 0 {
            index = atop(ruck[iter / 2]) as usize;
            items[index].0 = 1;
            if items[index].1 == 1 {
                break;
            }
        } else {
            index = atop(ruck[ruck.len() / 2 + (iter - 1) / 2]) as usize;
            items[index].1 = 1;
            if items[index].0 == 1 {
                break;
            }
        }
    }

    return index as u16;
}


fn atop(a: u8) -> u8 {
    return if a <= 90 { a - 64 + 26 } else { a - 70 - 26};
}