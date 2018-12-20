use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn main() ->std::io::Result<()> {
    let mut tot_set = HashSet::new();
    tot_set.insert(0);
    let mut total =  0;
    loop {
        
        let filename = "input";
        let mut f = File::open(filename)?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        contents.pop();

        let cord: Vec<&str> = contents.split("\n").collect();
        
        for i in cord {
            let operator = &i[..1];
            let num: i32 = i[1..].parse::<i32>().expect("error parsing num");
            if operator == "+" {
                total += num;
            } else {
                total -= num;
            }
            if !tot_set.contains(&total){
                tot_set.insert(total.clone());
            } else {
                println!("{}", total);
                return Ok(())

            }   
        }
    }

    Ok(())
}
