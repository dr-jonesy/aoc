use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn get_input() -> Result<String, std::io::Error> {
    let filename = "input";
    let mut f = File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    contents.pop();
    Ok(contents)
}

fn main() -> std::io::Result<()>{
    let xx = get_input()?;
    let data: Vec<&str> = xx.split("\n").collect();
    let mut doubles = 0;
    let mut triples = 0;

    for i in data {
        let mut doubs = 0;
        let mut trips = 0;
        let mut char_counter = HashMap::new();
        for ch in i.chars(){
            let counter = char_counter.entry(ch).or_insert(0);
            *counter += 1;
        }
        for val in char_counter.values(){
            if *val == 2 {
                doubs += 1;
            }
            if *val == 3 {
                trips += 1;
            }
        }

        if doubs >= 1 {
            doubles += 1;
        }
        if trips >= 1 {
            triples += 1;
        }
        println!("String: {}", i);
        for (key, value) in char_counter.iter(){
            if *value == 2 || *value == 3 {
                println!("\tchar: {}\tcount: {}", key, value);
            }
        }
    }
    let answer = doubles * triples;
    println!("{}", answer);
        

    Ok(())


}
