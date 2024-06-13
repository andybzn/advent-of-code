fn main() {
    let input: &str = include_str!("../input.txt");
    let answer: String = part_1(input);
    println!("Part 1: {answer}");
    dbg!(answer);
}

fn part_1(input: &str) -> String {
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
            .map(|vals| *vals.last().unwrap())
            .reduce(|acc, val| acc + val)
            .unwrap();
        totals.push(v);
    }
    let sum: i32 = totals.iter().sum();
    format!("{}", sum)
}

#[cfg(test)]
mod tests {
    use crate::part_1;
    #[test]
    fn sample_input_one_line() {
        let answer = part_1("0 3 6 9 12 15");
        assert_eq!(answer, "18".to_string())
    }

    #[test]
    fn sample_input() {
        let answer = part_1(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        assert_eq!(answer, "114".to_string())
    }

    #[test]
    fn actual_input() {
        let answer = part_1(include_str!("../input.txt"));
        assert_eq!(answer, "2075724761".to_string())
    }
}

