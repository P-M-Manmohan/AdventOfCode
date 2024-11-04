use std::fs;
use regex::Regex;


struct Game{
    red     : u32,
    green   : u32,
    blue    : u32,
}

fn possible (input: &str) -> u32 {
  
    let condition = Game{
        red:    12,
        green:  13,
        blue:   14,
    };

    let re = Regex::new(r"[ ;,:]+").unwrap();
    let mut flag =0;
    let mut game=0;
    let mut size =0;
    for token in re.split(input) {
        println!("{:?}",token);        
        match token {
            "Game"  =>  flag=1,
            "red"   =>  if size>condition.red{
                    return 0
            },
            "blue"  =>  if size>condition.blue{
                    return 0
            },
            "green" =>  if size>condition.green{
                    return 0
            },
            ""      => return game,
            _       => if flag==1{ 
                game = token.parse().unwrap(); 
                flag=0;
            }else{
                size= token.parse().unwrap();}
        }
    }
 return game;
}


fn main() {
    
    let contents = fs::read_to_string("../input.txt").expect("error reading file");

    let ans = contents
        .lines()
        .map(|game| possible(&game)).sum::<u32>();

    println!("{:?}",ans)

}
