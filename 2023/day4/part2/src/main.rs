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
    let ( hand, winning) = card.split_at(10);//10
    let mut wins = 0;
    for number in hand{
        if winning.contains(number) {
            wins+=1;

        }
    }
    wins

}

fn main() {
    let contents= fs::read_to_string("input").expect("read file does not exist");
    let mut cards: Vec<(Vec<String>,u32)> = contents.lines().map(|line| string_to_vec(line)).collect();
    let mut points = 0;
    let length = cards.len();
    let lotto = cards.clone();
    for card in cards.clone() {
        let wins = find_wins(card.clone());
        if wins!=0 {
            let index:usize = match lotto.iter().position(|n| *n == card) {
                Some(n) => n.try_into().unwrap(),
                _ => 0,
            };
            for num in (index+1)..(index+(wins as usize)+1) {
                if num<length{
                    cards[num].1+=cards[index].1;
                }
            }
        }

    }
    for card in cards{
        points+=card.1;
    }
    println!("{:?}",points)
}

