use std::fs::File;
use std::io::Read;

fn find_freq(cord: Vec<&str>) -> i32 {
    let mut total = 0;

    for i in cord{
        let op = &i[..1];
        let num: i32 = i[1..].parse::<i32>().unwrap();
        if op == "+"{
            total += num;
        } else {
            total -= num;
        }
    }
    
    total
}


fn main() ->std::io::Result<()> {
    let filename = "/home/hguant/projects/aoc/input.txt";
    let mut f = File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    contents.pop();
    
    let cord: Vec<&str> = contents.split("\n").collect();
    
    let answer = find_freq(cord);
    println!("{}", answer);
    Ok(())
}
