
use utils::read_lines;
use itertools::Itertools;


fn main() {

    let lines:Vec<String> = read_lines("input.txt").unwrap();

    //let part1_count = part1(lines.clone());

    let part2_count = part2(lines);




    println!("{:?}",  part2_count);

    test()
}

fn part1(lines: Vec<String>) -> i32 {
    let mut safe_count = 0;

    for line in lines {
        let mut report: Vec<i32> = line.split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        if report.is_sorted_by(|a, b| a < b && b - a < 4) {
            safe_count += 1;
        }

        if report.is_sorted_by(|a, b| a > b && a - b < 4) {
            safe_count += 1;
        }
    }
    safe_count
}

fn part2(lines: Vec<String>) -> i32 {
    let mut safe_count = 0;

    for line in lines {

        let mut report: Vec<i32> = line.split_whitespace().map(|i| i.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        if report.is_sorted_by(|a, b| a < b && b - a < 4) {
            safe_count += 1;
            continue
        }

        if report.is_sorted_by(|a, b| a > b && a - b < 4) {
            safe_count += 1;
            continue
        }

        let mut current_safe=0;

        for count in 0..report.len() {

            let mut tmp = report.clone();
            tmp.remove(count);

            if tmp.is_sorted_by(|a, b| a < b && b - a < 4) {
                current_safe += 1;
            }

            if tmp.is_sorted_by(|a, b| a > b && a - b < 4) {
                current_safe += 1;
            }
        }
        if current_safe==1 || current_safe==2{
            safe_count+=1;
            println!("{} : safe", line);
        }




    }
    safe_count
}


fn test(){

    let lines:Vec<String> = vec!["7 6 4 2 1".to_string(), "1 2 7 8 9".to_string(), "9 7 6 2 1".to_string(), "1 3 2 4 5".to_string(), "8 6 4 4 1".to_string(), "1 3 6 7 9".to_string()];

    println!("{}", part2(lines));
}
