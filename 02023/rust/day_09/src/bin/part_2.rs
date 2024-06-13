fn main() {
    println!("--- Day 9: Mirage Maintenance ---");
    let input: &str = include_str!("../input.txt");
    let answer: String = part_2(input);
    println!("Part 2: {answer}");
}

fn part_2(input: &str) -> String {
    let mut totals: Vec<i32> = Vec::new();
    for line in input.lines() {
        let mut readings: Vec<Vec<i32>> =
            Vec::from([line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect()]);
        loop {
            let r: Vec<i32> = readings
                .last()
                .unwrap()
                .windows(2)
                .map(|vals| vals[1] - vals[0])
                .collect();
            let zeroed: bool = r.iter().all(|x| *x == 0);
            readings.push(r);
            if zeroed {
                break
            }
        }

        readings.reverse();
        let v: i32 = readings.iter()
            .map(|vals| *vals.first().unwrap())
            .reduce(|acc, val| val - acc)
            .unwrap();
        totals.push(v);
    }
    let sum: i32 = totals.iter().sum();
    format!("{}", sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_input_one_line() {
        let answer = part_2("10 13 16 21 30 45");
        assert_eq!(answer, "5".to_string())
    }

    #[test]
    fn sample_input() {
        let answer = part_2(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        assert_eq!(answer, "2".to_string())
    }

    #[test]
    fn actual_input() {
        let answer = part_2(include_str!("../input.txt"));
        assert_eq!(answer, "1072".to_string())
    }
}

