use utils::read_lines;
use regex::Regex;

fn main() {

    let lines:Vec<String> = utils::read_lines("input.txt").unwrap();

       println!("{}",lines.len());

    //let line = line.get(0).unwrap().to_string();

    part2(lines);

}

fn part1(lines: Vec<String>) {
    let mut total = 0;

    for line in lines {
        let mul_re = Regex::new(r"mul\(([0-9]*),([0-9]*)\)").unwrap();

        let exemple = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let muls: Vec<u32> = mul_re.captures_iter(&*line).map(|caps| {
            let (_, [first, second]) = caps.extract();

            println!("{}, {}", first, second);

            first.parse::<u32>().unwrap() * second.parse::<u32>().unwrap()
        }
        ).collect();


        let res: u32 = muls.iter().sum();
        total += res;
    }
    println!("{:?}", total);
}

fn part2(lines: Vec<String>) {
    let mut total = 0;
    let mut capture:bool = true;

    for line in lines {
        let mul_re = Regex::new(r"mul\(([0-9]*),([0-9]*)\)|(don't\(\))|(do\(\))").unwrap();

        let exemple = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let small_re = Regex::new(r"mul\(([0-9]*),([0-9]*)\)").unwrap();

        let muls: Vec<&str> = mul_re.find_iter(&*line).map(|m| m.as_str()).collect();
        for instr in muls {

            if instr == "don't()" {
                capture = false;
            }

            if instr == "do()" {
                capture = true;
            }

            if capture {

                let res:Vec<u32> = small_re.captures_iter(&*instr).map(|caps| {
                    let (_, [first, second]) = caps.extract();

                    println!("{}, {}", first, second);

                    first.parse::<u32>().unwrap() * second.parse::<u32>().unwrap()
                }
                ).collect();


                let res: u32 = res.iter().sum();
                total += res;
            }
        }
    }
    println!("{:?}", total);
}


