use std::fs;
use regex::Regex;

fn string_to_vec(line: &str) -> (Vec<String>,u32){
    let re = Regex::new(r"\d+").unwrap();
    let mut cards: Vec<String> = re.find_iter(line)
        .map(|m| m.as_str().to_string())
        .collect();
    cards.remove(0);
    (cards,1) 
}

fn find_wins( cards:  (Vec<String>,u32)) -> u32 {
    let card = cards.0;
    let ( hand, winning) = card.split_at(10);
    let mut wins = 0;
    for number in hand{
        if winning.contains(number) {
            wins+=1;
        }
    }
    wins

}

fn main() {
    let contents= fs::read_to_string("test.txt").expect("read file does not exist");
    let mut cards: Vec<(Vec<String>,u32)> = contents.lines().map(|line| string_to_vec(line)).collect();
    println!("{:?}",cards);
    let mut wins =0;
    let mut points = 0;
    for card in cards {
        wins= find_wins(card.clone());
        if wins!=0 {
            points+=card.1;
            for _ in 0..wins{
                card.1
            }
        }

    }
    println!("{:?}",points)
}

