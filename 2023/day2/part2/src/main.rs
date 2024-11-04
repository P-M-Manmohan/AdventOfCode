use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Game{
    red     : u32,
    green   : u32,
    blue    : u32,
}

fn possible (input: &str) -> u32 {
  
    let mut max = Game{
        red:    0,
        green:  0,
        blue:   0,
    };

    let re = Regex::new(r"[ ;,:]+").unwrap();
    let mut size =0;
    for token in re.split(input) {
        match token {
            "Game"  =>  continue,
            "red"   =>  if size>max.red{
                    max.red=size;
            },
            "blue"  =>  if size>max.blue{
                    max.blue=size;
            },
            "green" =>  if size>max.green{
                    max.green=size;
            },
            ""      =>  continue,
            _       =>  size = token.parse().unwrap(),
        }
    }
    let power = max.red * max.green * max.blue;
    println!("{:?}",max);
    println!("{:?}", power);
    
return power
}


fn main() {
    
    let contents = fs::read_to_string("../input.txt").expect("error reading file");

    let ans = contents
        .lines()
        .map(|game| possible(&game)).sum::<u32>();

    println!("{:?}",ans)

}
