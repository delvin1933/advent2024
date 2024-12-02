
use utils::read_lines;

fn main() {

    let mut left:Vec<u32> = Vec::new();
    let mut right:Vec<u32> = Vec::new();

    let lines:Vec<String> = read_lines("input.txt").unwrap();
    for line in lines {
        let mut res = line.split_whitespace();
        left.push(res.next().unwrap().parse().unwrap());
        right.push(res.next().unwrap().parse().unwrap());
    }

    part2(left, right);


}

fn part1(mut left: Vec<u32>, mut right: Vec<u32>){

    left.sort();
    right.sort();

    let mut distances: Vec<u32> = Vec::new();
    let mut all_dist:u32 = 0;

    for i in 0..left.len(){

        let lefty = left.get(i).unwrap();
        let righty = right.get(i).unwrap();

        let distance = if righty > lefty {
            righty - lefty
        }else{
            lefty - righty
        };

        all_dist+=distance;

        distances.push( distance);
    }

    println!("part 1 : {:?}", all_dist);

}


fn part2(mut left: Vec<u32>, mut right: Vec<u32>){

    let mut results :Vec<(u32, u32)> = Vec::new();

    for lefty in left{

        let count = right.iter().filter(|&n| *n == lefty).count();
        if count != 0{
            println!("{}, {}", lefty, count);

            results.push((lefty, count as u32));
        }


    }



    let mut acc:u32 = 0;
    for (a, b) in results.iter(){
        acc+= a*b;
    }

    println!("{:?}", acc);



}