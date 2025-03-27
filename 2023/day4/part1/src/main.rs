use std::fs;
use regex::Regex;

fn string_to_vec(line: &str) -> Vec<String>{
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(line)
        .map(|m| m.as_str().to_string())
        .collect()
}

fn find_wins(mut card:  Vec<String>) -> u32 {
    card.remove(0);
    let ( hand, winning) = card.split_at(10);
    let mut wins = 0;
    let mut points = 0;
    for number in hand{
        if winning.contains(number) {
            wins+=1;
        }
    }
    for _number in 0..wins{
        if points== 0{
            points += 1;
        }
        else{
            points*=2;
        }
    }
    points

}

fn main() {
    let contents= fs::read_to_string("input").expect("read file does not exist");
    let cards: Vec<Vec<String>> = contents.lines().map(|line| string_to_vec(line)).collect();
    let mut points =0;
    for card in cards {
        points+= find_wins(card);
    }
    println!("{:?}",points)
}
